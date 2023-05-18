use crate::{errors::ApplicationError, tx::Tx};
use std::collections::HashMap;
use std::fmt;

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::NotFound(account) => write!(f, "Account {} not found", account),
            ApplicationError::UnderFunded(account, amount) => write!(
                f,
                "Account {} is underfunded; required amount is {}",
                account, amount
            ),
            ApplicationError::OverFunded(account, amount) => write!(
                f,
                "Account {} is overfunded; maximum allowed amount is {}",
                account, amount
            ),
        }
    }
}

impl std::error::Error for ApplicationError {}

/// A type for managing accounts and their current currency balance
#[derive(Debug)]
pub struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    /// Returns an empty instance of the [`Accounts`] type
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }

    /// Either deposits the `amount` provided into the `signer` account or adds the amount to the existing account.
    /// # Errors
    /// Attempted overflow
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .map(|r| *account = r)
                .ok_or(ApplicationError::OverFunded(signer.to_string(), amount))
                // Using map() here is an easy way to only manipulate the non-error result
                .map(|_| Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    /// Withdraws the `amount` from the `signer` account.
    /// # Errors
    /// Attempted overflow
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        if let Some(bal) = self.accounts.get_mut(signer) {
            (*bal)
                .checked_sub(amount)
                .map(|r| *bal = r)
                .ok_or(ApplicationError::UnderFunded(signer.to_string(), amount))
                .map(|_| Tx::Withdraw {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            Err(ApplicationError::NotFound(signer.to_string()))
        }
    }

    /// Withdraws the amount from the sender account and deposits it in the recipient account.
    ///
    /// # Errors
    /// The account doesn't exist
    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), ApplicationError> {
        let sender_previous_balance = *self
            .accounts
            .get(sender)
            .ok_or(ApplicationError::NotFound(sender.to_string()))?;

        match self.withdraw(sender, amount) {
            Ok(withdrawal_tx) => match self.deposit(recipient, amount) {
                Ok(deposit_tx) => Ok((withdrawal_tx, deposit_tx)),
                Err(ApplicationError::OverFunded(account, amount)) => {
                    // If the deposit fails due to OverFunded error,
                    // restore the sender's balance and return the error
                    *self.accounts.get_mut(sender).unwrap() = sender_previous_balance;
                    Err(ApplicationError::OverFunded(account, amount))
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Accounts;
    use super::*;

    #[test]
    fn test_withdraw_underfunded() {
        //arrange
        let mut ledger = Accounts::new();
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 50); // Insert a test account with balance 50

        //act
        match ledger.withdraw(signer, 100) {
            Ok(_) => panic!("Expected UnderFunded error, but got Ok(_)"),
            Err(e) => match e {
                ApplicationError::UnderFunded(account, amount) => {
                    assert_eq!(account, signer);
                    assert_eq!(amount, 100);
                }
                _ => panic!("Expected UnderFunded error, but got a different error"),
            },
        }
    }
    #[test]
    fn test_accounts_deposit_overfunded() {
        //arrange
        let mut ledger = Accounts::new();
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 50); // Insert a test account with balance 50

        //act
        match ledger.deposit(signer, std::u64::MAX) {
            Ok(_) => panic!("Expected OverFunded error, but got Ok(_)"),
            Err(e) => match e {
                ApplicationError::OverFunded(account, amount) => {
                    assert_eq!(account, signer);
                    assert_eq!(amount, 18446744073709551615);
                }
                _ => panic!("Expected UnderFunded error, but got a different error"),
            },
        }
    }

    #[test]
    fn test_accounts_deposit_works() {
        //arrange
        let mut ledger = Accounts::new();
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 0);

        //act
        match ledger.deposit(signer, 100) {
            Ok(_) => assert_eq!(*ledger.accounts.get("test_account").unwrap(), 100),
            Err(e) => panic!("Expected deposit to work but got error{:?}", e),
        }
    }

    #[test]
    fn test_accounts_withdraw_works() {
        //arrange
        let mut ledger = Accounts::new();
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 100);

        //act
        match ledger.withdraw(signer, 100) {
            Ok(_) => assert_eq!(*ledger.accounts.get("test_account").unwrap(), 0),
            Err(e) => panic!("Expected deposit to work but got error{:?}", e),
        }
    }

    #[test]
    fn test_accounts_send_works() {
        let mut ledger = Accounts::new();
        let sender = "test_account";
        let receiver = "test_account2";
        ledger.accounts.insert(sender.to_string(), 100);
        ledger.accounts.insert(receiver.to_string(), 0);

        //act
        match ledger.send(sender, receiver, 100) {
            Ok(_) => assert_eq!(*ledger.accounts.get("test_account2").unwrap(), 100),
            Err(e) => panic!("Expected deposit to work but got error{:?}", e),
        }
    }

    #[test]
    fn test_accounts_send_underfunded_fails_and_rolls_back() {
        let mut ledger = Accounts::new();
        let sender = "test_account";
        let receiver = "test_account2";
        ledger.accounts.insert(sender.to_string(), 10);
        ledger.accounts.insert(receiver.to_string(), 0);

        //act
        match ledger.send(sender, receiver, 100) {
            Ok(tx) => panic!("Expected send to fail but but succeeded. Tx:{:?}", tx),
            Err(e) => match e {
                ApplicationError::UnderFunded(sender, 100) => {
                    assert_eq!(*ledger.accounts.get(&sender).unwrap(), 10)
                }
                _ => panic!("Expected UnderFunded error, but got a different error"),
            },
        };
    }

    #[test]
    fn test_accounts_send_overfunded_fails_and_rolls_back() {
        let mut ledger = Accounts::new();
        let sender = "test_account";
        let receiver = "test_account2";
        ledger.accounts.insert(sender.to_string(), std::u64::MAX);
        ledger.accounts.insert(receiver.to_string(), 10);

        //act
        match ledger.send(sender, receiver, std::u64::MAX) {
            Ok(tx) => panic!("Expected send to fail but but succeeded. Tx:{:?}", tx),
            Err(e) => match e {
                ApplicationError::OverFunded(sender, 18446744073709551615) => {
                    assert_eq!(*ledger.accounts.get(&sender).unwrap(), 10)
                }
                _ => panic!("Expected OverFunded error, but got a different error"),
            },
        };
    }
}
