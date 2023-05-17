use crate::{errors::AccountingError, tx::Tx};
use std::collections::HashMap;
use std::fmt;

impl fmt::Display for AccountingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountingError::NotFound(account) => write!(f, "Account {} not found", account),
            AccountingError::UnderFunded(account, amount) => write!(
                f,
                "Account {} is underfunded; required amount is {}",
                account, amount
            ),
            AccountingError::OverFunded(account, amount) => write!(
                f,
                "Account {} is overfunded; maximum allowed amount is {}",
                account, amount
            ),
        }
    }
}

impl std::error::Error for AccountingError {}

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
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .map(|r| *account = r)
                .ok_or(AccountingError::OverFunded(signer.to_string(), amount))
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
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(bal) = self.accounts.get_mut(signer) {
            (*bal)
                .checked_sub(amount)
                .map(|r| *bal = r)
                .ok_or(AccountingError::UnderFunded(signer.to_string(), amount))
                .map(|_| Tx::Withdraw {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            Err(AccountingError::NotFound(signer.to_string()))
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
    ) -> Result<(Tx, Tx), AccountingError> {
        Ok((
            self.withdraw(sender, amount)?,
            self.deposit(recipient, amount)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Accounts;
    use super::*;

    #[test]
    fn test_withdraw_underfunded() {
        let mut ledger = Accounts::new(); // Assuming you have a Ledger struct
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 50); // Insert a test account with balance 50

        match ledger.withdraw(signer, 100) {
            Ok(_) => panic!("Expected UnderFunded error, but got Ok(_)"),
            Err(e) => match e {
                AccountingError::UnderFunded(account, amount) => {
                    assert_eq!(account, signer);
                    assert_eq!(amount, 100);
                }
                _ => panic!("Expected UnderFunded error, but got a different error"),
            },
        }
    }
    #[test]
    fn test_accounts_deposit_overfunded() {
        todo!();
    }

    #[test]
    fn test_accounts_deposit_works() {
        let mut ledger = Accounts::new(); // Assuming you have a Ledger struct
        let signer = "test_account";
        ledger.accounts.insert(signer.to_string(), 0); // Insert a test account with balance 50

        match ledger.deposit(signer, 100) {
            Ok(_) => assert_eq!(*ledger.accounts.get("test_account").unwrap(), 100),
            Err(e) => panic!("Expected deposit to work but got error{:?}", e),
        }
    }

    #[test]
    fn test_accounts_withdraw_works() {
        todo!();
    }

    #[test]
    fn test_accounts_send_works() {
        todo!();
    }

    #[test]
    fn test_accounts_send_underfunded_fails_and_rolls_back() {
        todo!();
    }

    #[test]
    fn test_accounts_send_overfunded_fails_and_rolls_back() {
        todo!();
    }
}
