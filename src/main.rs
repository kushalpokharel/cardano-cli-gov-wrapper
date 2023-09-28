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
use utils::{status_check, query_utxo};
use crate::governance::governance::{Network, Governance, Constitution};
use crate::transaction::Transaction;
use crate::keypairs::KeyPair;
use crate::balance::maintain_balance;

#[tokio::main]
async fn main() {
    //setup necessary addresses, payment, stake and drep addresses
    let config = utils::load_env();

    fs::create_dir_all("./wallet").expect("Failed to create wallet directory");
    fs::create_dir_all("./gov-actions").expect("Failed to create gov action directory");
    fs::create_dir_all("./transactions").expect("Failed to create transactions directory");

    let wallet = setup::setup_wallet(&config);

    let mut govaction = Constitution::new(Network::Sancho, 0, String::from("https://shorturl.at/xMS15"), String::from("3d2a9d15382c14f5ca260a2f5bfb645fe148bfe10c1d0e1d305b7b1393e2bd97"), "https://shorturl.at/asIJ6".to_string(), String::from("./gov-actions/constitution.txt")  , String::from("./gov-actions/constitution.action"), String::from(""));
    let consti = govaction.create_constitution().await;
    utils::generic_result_check(consti, String::from("Constitution file creation"));

    //get all the utxos of the address, find their total balance, determine whether it is enough for the below transactions
    let address = match utils::load_file_contents(wallet.get_paddr_file()){
        Some(address) =>{
            address
        },
        None =>{
            println!("Could not load address file");
            std::process::exit(0);
        }
    };
    let mut tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./transactions/tx.raw"), String::from("./transactions/tx.signed"), String::from("action.file"));

    
    maintain_balance(address.clone(), &config, &mut tx).await;    

    //cardano-cli transaction building ()
    let status = govaction.create_action(&wallet, &config.network);
    status_check(status, String::from("Governance action generation"));
    let build_status = tx.build_tx(&wallet,String::from("--proposal-file"), &config, String::from("./gov-actions/constitution.action"), address.clone());
    status_check(build_status, String::from("Tx build file generation"));
    let id_status = tx.get_tx_id();
    status_check(id_status, String::from("TxId construction"));
    let sign_status = tx.sign_tx(&wallet);
    status_check(sign_status, String::from("Tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(&config);
    status_check(submit_status, String::from("Tx submission"));
}


