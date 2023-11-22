use std::process::ExitStatus;

use crate::config::Config;
use crate::utils::{status_check, generic_result_check, set_interval, check_for_utxo};
use crate::vote::generate_vote_file;
use crate::wallet::Wallet;
use crate::governance::governance::{Governance, Constitution};
use crate::transaction::Transaction;

pub async fn constitution_update(mut tx:Transaction ,wallet:&Wallet, config:&Config, address:String, proposal_url:String, proposal_hash:String, constitution_file:String, constitution_url:String, output_file:String, number:u32 )-> Result<String, String>{
    let mut govaction = Constitution::new( 0, proposal_url, proposal_hash,  constitution_url, constitution_file, output_file, String::from(""));
    let consti = govaction.create_constitution().await?;
    status_check(govaction.create_action(wallet, &config.network), String::from("Governance action generation"))?;
    let build_status = tx.build_tx(wallet,String::from("--proposal-file"), config, govaction.get_out_file(), String::from("1"),address.clone());
    status_check(build_status, String::from("Governance action tx build file generation"))?;
    let id = tx.get_tx_id()?;
    let sign_status = tx.sign_tx(wallet);
    status_check(sign_status, String::from("Governance action tx Sign"))?;
    //submit the transaction
    let submit_status = tx.submit_tx(config);
    status_check(submit_status, String::from("Governance action tx submission")+&number.to_string())?;
    set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id.clone(), address).await;
    println!("Governance action number {} transaction confirmed", number);
    Ok(id)
}

pub async fn send_ada(mut tx:Transaction, send_wallet:&Wallet, config:&Config, send_address:String)->Result<String,String>{
    let build_status = tx.build_tx(send_wallet, String::from(""), config, String::from(""), String::from("1"), send_address.clone());
    status_check(build_status, String::from("Send ada tx build file generation"))?;
    let id = tx.get_tx_id()?;
    println!("Send ada Transaction id = {}", id);
    let sign_status = tx.sign_tx(send_wallet);
    status_check(sign_status, String::from("Send ada tx Sign"))?;
    //submit the transaction
    let submit_status = tx.submit_tx(config);
    check_submit_status(submit_status, id.clone(), send_address).await;
    Ok(id)
}

pub async fn drep_register(mut tx:Transaction ,wallet:&Wallet, config:&Config, address:String){

    let build_status = tx.build_tx(wallet, String::from("--certificate-file"), config, wallet.get_drep_file().get_certificate(), String::from("2"), address.clone());
    status_check(build_status, String::from("Drep registration tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Drep registration txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx(wallet);
    status_check(sign_status, String::from("Drep registration tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(config);
    check_submit_status(submit_status, id, address).await;
    
}

pub async fn vote_proposal(mut vote_tx:Transaction, action_id:String, wallet:&Wallet, config:&Config, address:String){
    generate_vote_file(String::from("yes"), String::from(action_id.trim()), String::from("./files/vote/vote"), String::from("0"), wallet).expect("Vote file generation unsuccessful");
    let build_status = vote_tx.build_tx(wallet, String::from("--vote-file"), config, String::from("./files/vote/vote"), String::from("2"), address.clone());
    status_check(build_status, String::from("Voting tx build file generation"));
    let tx_id = vote_tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Vote txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = vote_tx.sign_tx_keys(vec![wallet.get_drep_file().get_private_path(), wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Vote tx Sign"));
    //submit the transaction
    let submit_status = vote_tx.submit_tx(config);
    check_submit_status(submit_status, id, address).await;
    
}

pub async fn delegate_drep(mut tx:Transaction ,holder_wallet:&Wallet, drep_wallet:&Wallet, config:&Config, address:String){
    drep_wallet.get_drep_file().delegate_to_me(holder_wallet.get_skey_file().get_public_path(), String::from("./files/wallet/deleg.cert")).expect("Delegation certificate creation not successful");
    let build_status = tx.build_tx(holder_wallet, String::from("--certificate-file"), config, String::from("./files/wallet/deleg.cert"), String::from("2"), address.clone());
    status_check(build_status, String::from("Drep delegation tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Drep delegation txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx_keys(vec![holder_wallet.get_skey_file().get_private_path(), holder_wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Drep delegation tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(config);
    check_submit_status(submit_status, id, address).await;
    
}

pub async fn stake_reg_tx(mut tx:Transaction ,holder_wallet:&Wallet, config:&Config, address:String){
    let cert_status = holder_wallet.get_skey_file().gen_cert(200000);
    status_check(cert_status, String::from("Stake registration tx build file generation"));
    let build_status = tx.build_tx(holder_wallet, String::from("--certificate-file"), config, holder_wallet.get_skey_file().get_cert(), String::from("2"), address.clone());
    status_check(build_status, String::from("Stake registration tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Stake registration txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx_keys(vec![holder_wallet.get_skey_file().get_private_path(), holder_wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Stake registration tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(config);
    check_submit_status(submit_status, id, address).await;
}

async fn check_submit_status(submit_status:Result<ExitStatus, std::io::Error>, id:String, address:String){
    match submit_status{
        Ok(status) => {
            if status.success(){
                let utxo = set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id, address).await;
            }
            else{
                //todo::get the output and check if the error is alreadyregistereddrep otherwise just cancel the subsequent txs.
            }
        },
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        },
    }
}