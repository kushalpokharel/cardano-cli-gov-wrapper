use std::fs;
use crate::{config::Config, wallet::Wallet, utils::status_check, keypairs::{PaymentKeyFile, KeyPair, StakeKeyFile, DrepKeyFile }};

pub fn setup_wallet(conf:&Config)->Wallet{
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
                    println!("Payment keys already present");
                }
                Err(_)=>{
                    let status = pkf.gen_public();
                    status_check(status, String::from("Payment public key file generation"))

                }
            }
        }
        Err(_) => {
            let status = pkf.gen_pair();
            status_check(status, String::from("Payment key pair file generation"))
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
                    status_check(status, String::from("Stake public key file generation"));
                }
            }
        }
        Err(_) => {
            let status = skf.gen_pair();
            status_check(status, String::from("Stake key pair generation"));
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
                    status_check(status, String::from("DRep public key file generation"));
                }
            }
        }
        Err(_) => {
            let status = dkf.gen_pair();
            status_check(status, String::from("DRep key pair generation"));
        }
    }
    let status = dkf.gen_cert();
    status_check(status, String::from("DRep certificate generation"));
    dkf
}
