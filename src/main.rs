use std::fs::{File, self};
use std::io::Read;
// use std::process::Command;

mod governance;
mod transaction;
mod keypairs;
mod config;
mod wallet;
mod utils;
mod query;
use keypairs::{PaymentKeyFile, StakeKeyFile, DrepKeyFile};
use utils::status_check;
use wallet::Wallet;
use crate::governance::governance::{Network, Governance, Constitution};
use crate::transaction::Transaction;
use crate::keypairs::KeyPair;
use crate::config::Config;
use tokio::runtime;

fn main() {
    //setup necessary addresses, payment, stake and drep addresses
    let config = utils::load_env();
    let wallet = setup_wallet(&config);
    println!("{}",config.network);

    // //cardano-cli transaction building ()
    let mut tx = Transaction::new(Network::Sancho, vec![], "".to_string(), vec![], String::from("./transactions/tx.raw"), String::from("./transactions/tx.signed"), String::from("action.file"));

    let mut govaction = Constitution::new(Network::Sancho, 100000, String::from("https://shorturl.at/xMS15"), String::from("3d2a9d15382c14f5ca260a2f5bfb645fe148bfe10c1d0e1d305b7b1393e2bd97"), "https://shorturl.at/asIJ6".to_string(), String::from("./gov-actions/constitution.txt")  , String::from("./gov-actions/constitution.action"), String::from(""));
    let rt = runtime::Runtime::new().unwrap();
    let consti = govaction.create_constitution();
    let res = rt.block_on(consti);
    utils::generic_result_check(res, String::from("Constitution file creation"));
    let status = govaction.create_action(&wallet, &config.network);
    status_check(status, String::from("Governance action generation"));
    tx.build_tx(&wallet,String::from("--constitution-file"), &config.network, String::from("./gov-actions/constitution.action"));
    tx.get_tx_id();
    tx.sign_tx(&wallet);
    // // //submit the transaction
    tx.submit_tx();
}


fn setup_wallet(conf:&Config)->Wallet{
    let pkf = setup_payment_key_pair(conf);
    let skf = setup_stake_key_pair(conf);
    let dkf = setup_drep_key_pair(conf);
    let wallet = Wallet::new(pkf, skf, dkf, String::from("./wallet/address"));
    let status = wallet.gen_addr(conf.network.clone());
    status_check(status, String::from("Address generation"));
    return wallet;
}

fn setup_payment_key_pair(conf:&Config)->PaymentKeyFile{
    let payment_pubkey = String::from("./wallet/payment.pub");
    let payment_privatekey = String::from("./wallet/payment.priv");

    let pkf = PaymentKeyFile::new(payment_pubkey.clone(), payment_privatekey.clone());
    
    match fs::metadata(payment_privatekey){
        Ok(_) => {
            match fs::metadata(payment_pubkey){
                Ok(_) => {
                    println!("both files present");
                }
                Err(_)=>{
                    let status = pkf.gen_public();
                    utils::status_check(status, String::from("Payment public key file generation"))

                }
            }
        }
        Err(_) => {
            let status = pkf.gen_pair();
            utils::status_check(status, String::from("Payment key pair file generation"))
        }
    }
    pkf
    
}

fn setup_stake_key_pair(conf:&Config)->StakeKeyFile{
    let stake_pubkey = String::from("./wallet/stake.pub");
    let stake_privkey = String::from("./wallet/stake.priv");

    let skf = StakeKeyFile::new(stake_pubkey.clone(), stake_privkey.clone());
    
    match fs::metadata(stake_privkey){
        Ok(_) => {
            match fs::metadata(stake_pubkey){
                Ok(_) => {
                    println!("both files present");
                }
                Err(_)=>{
                    let status = skf.gen_public();
                    utils::status_check(status, String::from("Stake public key file generation"));
                }
            }
        }
        Err(_) => {
            let status = skf.gen_pair();
            utils::status_check(status, String::from("Stake key pair generation"));
        }
    }
    skf
}

fn setup_drep_key_pair(conf:&Config)->DrepKeyFile{
    let drep_pubkey = String::from("./wallet/drep.pub");
    let drep_privkey = String::from("./wallet/drep.priv");
    let drep_certificate:String = String::from("./wallet/drep.cert");
    let dkf = DrepKeyFile::new(drep_pubkey.clone(), drep_privkey.clone(), drep_certificate.clone());
    
    match fs::metadata(drep_privkey){
        Ok(_) => {
            match fs::metadata(drep_pubkey){
                Ok(_) => {
                    println!("both files present");
                }
                Err(_)=>{
                    let status = dkf.gen_public();
                    utils::status_check(status, String::from("DRep public key file generation"));
                }
            }
        }
        Err(_) => {
            let status = dkf.gen_pair();
            utils::status_check(status, String::from("DRep key pair generation"));
        }
    }
    let status = dkf.gen_cert();
    utils::status_check(status, String::from("DRep certificate generation"));
    dkf
}


