use std::{io, println, todo};
mod accounts;

fn main() {
    loop {
        let input = read_from_stdin(
            "Choose operation [deposit, withdraw, send, print, quit], confirm with return:",
        );

        match input.as_str() {
            "deposit" => todo!(),
            "withdraw" => todo!(),
            "send" => todo!(),
            "print" => todo!(),
            "quit" => todo!(),
            _ => todo!(),
        }
    }
    // println!("Hello, accounting world!");

    // // We are using simple &str instances as keys
    // // for more sophisticated keys (e.g. hashes)
    // // the data type could remain the same
    // let bob = "bob";
    // let alice = "alice";
    // let charlie = "charlie";s
    // let initial_amount = 100;

    // // Creates the basic ledger and a tx log container
    // let mut ledger = Accounts::new();
    // let mut tx_log = vec![];

    // // Deposit an amount to each account
    // for signer in &[bob, alice, charlie] {
    //     let status = ledger.deposit(signer, initial_amount);
    //     println!("Depositing {} for {}: {:?}", signer, initial_amount, status);
    //     // Add the resulting transaction to a list of transactions
    //     // .unwrap() will crash the program if the status is an error.
    //     tx_log.push(status.unwrap());
    // }

    // // Send currency from one account (bob) to the other (alice)
    // let send_amount = 10_u64;
    // let status = ledger.send(bob, alice, send_amount);
    // println!(
    //     "Sent {} from {} to {}: {:?}",
    //     send_amount, bob, alice, status
    // );

    // // Add both transactions to the transaction log
    // let (tx1, tx2) = status.unwrap();
    // tx_log.push(tx1);
    // tx_log.push(tx2);

    // // Withdraw everything from the accounts
    // let tx = ledger.withdraw(charlie, initial_amount).unwrap();
    // tx_log.push(tx);
    // let tx = ledger
    //     .withdraw(alice, initial_amount + send_amount)
    //     .unwrap();
    // tx_log.push(tx);

    // // Here we are withdrawing too much and there won't be a transaction
    // println!(
    //     "Withdrawing {} from {}: {:?}",
    //     initial_amount,
    //     bob,
    //     ledger.withdraw(bob, initial_amount)
    // );
    // // Withdrawing the expected amount results in a transaction
    // let tx = ledger.withdraw(bob, initial_amount - send_amount).unwrap();
    // tx_log.push(tx);

    // // {:?} prints the Debug implementation, {:#?} pretty-prints it
    // println!("Ledger empty: {:?}", ledger);
    // println!("The TX log: {:#?}", tx_log);
}

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}
