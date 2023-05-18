use crate::{accounts::Accounts, tx::Tx};
use std::{io, println};
mod accounts;
mod core;
mod errors;
mod tx;

enum InputResult {
    Quit,
    Print,
    Confirmed(Vec<Tx>),
    NotSupported,
}

fn main() {
    // Creates the basic ledger and a tx log container
    let mut ledger = Accounts::new();
    let mut tx_log = vec![];

    loop {
        match handle_input(&mut ledger) {
            Ok(InputResult::Confirmed(mut tx)) => {
                tx_log.append(&mut tx);
                continue;
            }
            Ok(InputResult::Quit) => break,
            Err(e) => println!("encountered error: {}", e),
            _ => continue,
        }
    }
}

fn handle_input(ledger: &mut Accounts) -> Result<InputResult, Box<dyn std::error::Error>> {
    let input =
        read_from_stdin("Please choose [deposit, withdraw, send, print, quit] and  hit return:");

    match input.as_str() {
        "deposit" => {
            let account = read_from_stdin("Account:");
            let amount: u64 = read_from_stdin("Amount").parse()?;
            let tx = ledger.deposit(&account, amount)?;
            Ok(InputResult::Confirmed(vec![tx]))
        }
        "withdraw" => {
            let account = read_from_stdin("Account:");
            let amount: u64 = read_from_stdin("Amount").parse()?;
            let tx = ledger.withdraw(&account, amount)?;
            Ok(InputResult::Confirmed(vec![tx]))
        }
        "send" => {
            let sender = read_from_stdin("Sender:");
            let amount: u64 = read_from_stdin("Amount").parse().unwrap();
            let receiver = read_from_stdin("Receiver");
            let (tx1, tx2) = ledger.send(&sender, &receiver, amount)?;
            Ok(InputResult::Confirmed(vec![tx1, tx2]))
        }
        "print" => {
            println!("ledger: {:?}", ledger);
            Ok(InputResult::Print)
        }
        "quit" => Ok(InputResult::Quit),
        _ => {
            println!("command not supported");
            Ok(InputResult::NotSupported)
        }
    }
}

fn read_from_stdin(label: &str) -> String {
    let mut buffer = String::new();
    println!("{}", label);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    buffer.trim().to_owned()
}
