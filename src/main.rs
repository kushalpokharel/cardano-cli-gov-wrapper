use std::fs::{self};

mod governance;
mod transaction;
mod keypairs;
mod config;
mod wallet;
mod utils;
mod query;
mod setup;
mod balance;
mod vote;
mod actions;
use wallet::Wallet;

use crate::governance::governance::{Network};
use crate::transaction::Transaction;
use crate::balance::maintain_balance;

#[tokio::main]
async fn main() {
    //setup necessary addresses, payment, stake and drep addresses
    let config: config::Config = utils::load_env();
    create_folders();
    let drep_wallet = setup::setup_wallet(String::from(""),&config);
    let drep_address = get_address(&drep_wallet);

    let ada_holder_wallet = setup::setup_wallet(String::from("ada_holder"),&config);
    let holder_address = get_address(&ada_holder_wallet);

    let mut tx: Transaction = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./files/transactions/const_tx.raw"), String::from("./files/transactions/const_tx.signed"), String::from("action.file"));
    let utxos = maintain_balance(drep_address.clone(), &config).await; 
    tx.add_input_list(&utxos);   
    let txid = actions::constitution_update(tx, &drep_wallet, &config, drep_address.clone()).await;

    println!();

    let mut reg_tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./files/transactions/reg_tx.raw"), String::from("./files/transactions/reg_tx.signed"), String::from("action.file"));
    let utxos = maintain_balance(drep_address.clone(), &config).await; 
    reg_tx.add_input_list(&utxos);   
    actions::drep_register(reg_tx, &drep_wallet, &config, drep_address.clone()).await;

    println!();

    let mut stake_reg_tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./files/transactions/stake_tx.raw"), String::from("./files/transactions/stake_tx.signed"), String::from("action.file"));
    let utxos = maintain_balance(holder_address.clone(), &config).await; 
    stake_reg_tx.add_input_list(&utxos);   
    actions::stake_reg_tx(stake_reg_tx, &ada_holder_wallet, &config, holder_address.clone()).await;

    println!();

    let mut deleg_tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./files/transactions/deleg_tx.raw"), String::from("./files/transactions/deleg_tx.signed"), String::from("action.file"));
    let utxos = maintain_balance(holder_address.clone(), &config).await; 
    deleg_tx.add_input_list(&utxos);   
    actions::delegate_drep(deleg_tx, &ada_holder_wallet, &drep_wallet, &config, holder_address).await;

    println!();
    let mut vote_tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./files/transactions/vote_tx.raw"), String::from("./files/transactions/vote_tx.signed"), String::from("action.file"));
    let utxos = maintain_balance(drep_address.clone(), &config).await; 
    vote_tx.add_input_list(&utxos);   
    actions::vote_proposal(vote_tx, txid, &drep_wallet, &config, drep_address).await;    

}

fn create_folders(){
    fs::create_dir_all("./files").expect("Failed to create files directory");
    fs::create_dir_all("./files/wallet").expect("Failed to create wallet directory");
    fs::create_dir_all("./files/gov-actions").expect("Failed to create gov action directory");
    fs::create_dir_all("./files/transactions").expect("Failed to create transactions directory");
    fs::create_dir_all("./files/vote").expect("Failed to create vote directory");
}

fn get_address(wallet:&Wallet)->String{
    match utils::load_file_contents(wallet.get_paddr_file()){
        Some(address) =>{
            address
        },
        None =>{
            println!("Could not load address file");
            std::process::exit(0);
        }
    }
}