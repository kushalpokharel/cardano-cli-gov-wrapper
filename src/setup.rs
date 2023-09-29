use std::{fs};
use crate::{config::Config, wallet::Wallet, utils::status_check, keypairs::{PaymentKeyFile, KeyPair, StakeKeyFile, DrepKeyFile }};

pub fn setup_wallet(prefix:String, conf:&Config)->Wallet{
    let pkf = setup_payment_key_pair(prefix.clone(),conf);
    let skf = setup_stake_key_pair(prefix.clone(), conf);
    let dkf = setup_drep_key_pair(prefix.clone(), conf);
    let wallet_address = String::from("./wallet/")+  &prefix +&String::from("address");
    let wallet = Wallet::new(pkf, skf, dkf, wallet_address );
    let status = wallet.gen_addr(conf.network.clone());
    status_check(status, prefix+&String::from("Address generation"));
    return wallet;
}

fn setup_payment_key_pair(prefix:String, conf:&Config)->PaymentKeyFile{
    let payment_pubkey = String::from("./wallet/")+&prefix+&String::from("payment.pub");
    let payment_privatekey = String::from("./wallet/")+&prefix+&String::from("payment.priv");

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

fn setup_stake_key_pair(prefix:String, conf:&Config)->StakeKeyFile{
    let stake_pubkey = String::from("./wallet/")+&prefix +&String::from("stake.pub");
    let stake_privkey = String::from("./wallet/")+&prefix+ &String::from("stake.priv");
    let stake_cert = String::from("./wallet/")+&prefix+ &String::from("stake.cert");

    let skf = StakeKeyFile::new(stake_pubkey.clone(), stake_privkey.clone(), stake_cert.clone());
    
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

fn setup_drep_key_pair(prefix:String, conf:&Config)->DrepKeyFile{
    let drep_pubkey = String::from("./wallet/drep.pub");
    let drep_privkey = String::from("./wallet/drep.priv");
    let drep_certificate:String = String::from("./wallet/drep.cert");
    let drep_id:String = String::from("./wallet/drep.id");
    let dkf = DrepKeyFile::new(drep_pubkey.clone(), drep_privkey.clone(), drep_certificate.clone(), drep_id.clone());
    
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
    let mut status = dkf.gen_cert();
    status_check(status, String::from("DRep certificate generation"));
    status = dkf.gen_id();
    status_check(status, String::from("DRep id generation"));
    dkf
}
