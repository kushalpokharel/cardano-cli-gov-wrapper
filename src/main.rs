use std::fs::File;
use std::io::Read;
use std::process::Command;

mod governance;
mod transaction;
use crate::governance::governance::{Network, Governance};
use crate::transaction::Transaction;

fn main() {
    //setup necessary addresses, payment, stake and drep addresses

    //cardano-cli transaction building ()
    let mut tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], "addr_test1qqvaapk7a6dzly5dez0g4655rvqp4q4dape7ksg5dqn8es7q49dk044tafaf0mg6uk8jpx7tpczu950qz7ck355k854syglwys".to_string(), "tx.raw".to_string(), "tx.signed".to_string(), "action.file".to_string() );
    let mut file = File::open("constitution.txt").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    let govaction = Governance::new(Network::Sancho, 100000, "stake.vkey".to_string(), "https://shorturl.at/xMS15".to_string(), "3d2a9d15382c14f5ca260a2f5bfb645fe148bfe10c1d0e1d305b7b1393e2bd97".to_string(), "https://shorturl.at/asIJ6".to_string(), buf  , "action.file".to_string(), "create-constitution".to_string());
    govaction.create_action();
    tx.add_input("e0c8cf1dacc325137f46440009bd60b5f9128a2aac8c9029ac8cf28485be3686#0".to_string());
    tx.build_tx();
    tx.get_tx_id();
    tx.sign_tx();
    //submit the transaction
    tx.submit_tx();
}

