use std::fs::{self};

mod governance;
mod transaction;
mod keypairs;
mod config;
mod wallet;
mod utils;
mod setup;
mod balance;
mod vote;
mod actions;
mod proposal;
use balance::{maintain_balance_from_account, send_balance};
// use balance::maintain_balance_from_account;
use config::Config;
use wallet::Wallet;

use crate::transaction::Transaction;


#[tokio::main]
async fn main() {
    let configres= utils::load_env();
    let config = match configres{
        Ok(x) => x,
        Err(err) => {
            println!("Config not loaded successfully: {}", err);
            std::process::exit(0);
        }
    };
    let config = std::sync::Arc::new(config);
    match setup::create_folders(){
        Ok(_) => (),
        Err(err) => {
            println!("Folder generation not successful: {}", err.to_string() );
            std::process::exit(0);
        }
    };
    match proposal::make_proposal(&config, 10).await{
        Ok((success,_failure)) => {
            println!("The process ended with {} successful transactions with txid below", success.len());
            success.iter().enumerate().for_each(|(i, tx_id)| {
                print!("Transaction {} with id {}", i, tx_id);
            });
        },
        Err(err) => {
            println!("Proposal generation not successful: {}", err.to_string() );
            std::process::exit(0);
        },
    };
    // make_proposal_and_vote(5).await

    //setup necessary addresses, payment, stake and drep addresses
    

}

// pub async fn make_proposal_and_vote( config:&config::Config, number_of_proposals:u32){
    
//     let drep_wallet = setup::setup_wallet(String::from("drep"),&config);
//     let drep_address: String = get_address(&drep_wallet);

//     let ada_holder_wallet = setup::setup_wallet(String::from("ada_holder"),&config);
//     let holder_address = get_address(&ada_holder_wallet);

//     println!();

//     let mut tx: Transaction = Transaction::new(config.network.clone(), vec![], String::from("./files/transactions/const_tx.raw"), String::from("./files/transactions/const_tx.signed"));
//     let utxos = maintain_balance(drep_address.clone(), &config).await; 
//     tx.add_input_list(&utxos);   
//     let txid = actions::constitution_update(tx, &drep_wallet, &config, drep_address.clone()).await;

//     println!();

//     let mut reg_tx = Transaction::new(config.network.clone(), vec![], String::from("./files/transactions/reg_tx.raw"), String::from("./files/transactions/reg_tx.signed"));
//     let utxos = maintain_balance(drep_address.clone(), &config).await; 
//     reg_tx.add_input_list(&utxos);   
//     actions::drep_register(reg_tx, &drep_wallet, &config, drep_address.clone()).await;

//     println!();

//     let mut stake_reg_tx = Transaction::new(config.network.clone(), vec![], String::from("./files/transactions/stake_tx.raw"), String::from("./files/transactions/stake_tx.signed"));
//     let utxos = maintain_balance(holder_address.clone(), &config).await; 
//     stake_reg_tx.add_input_list(&utxos);   
//     actions::stake_reg_tx(stake_reg_tx, &ada_holder_wallet, &config, holder_address.clone()).await;

//     println!();

//     let mut deleg_tx = Transaction::new(config.network.clone(), vec![], String::from("./files/transactions/deleg_tx.raw"), String::from("./files/transactions/deleg_tx.signed"));
//     let utxos = maintain_balance(holder_address.clone(), &config).await; 
//     deleg_tx.add_input_list(&utxos);   
//     actions::delegate_drep(deleg_tx, &ada_holder_wallet, &drep_wallet, &config, holder_address).await;

//     println!();
//     let mut vote_tx = Transaction::new(config.network.clone(), vec![], String::from("./files/transactions/vote_tx.raw"), String::from("./files/transactions/vote_tx.signed"));
//     let utxos = maintain_balance(drep_address.clone(), &config).await; 
//     vote_tx.add_input_list(&utxos);   
//     actions::vote_proposal(vote_tx, txid, &drep_wallet, &config, drep_address).await;    
// }

fn get_address(wallet:&Wallet)->String{
    match utils::load_file_contents(wallet.get_paddr_file()){
        Ok(address) =>{
            address
        },
        Err(_) =>{
            println!("Could not load address file");
            std::process::exit(0);
        }
    }
}