use crate::config::Config;
use crate::utils::{status_check, query_utxo, generic_result_check, set_interval, check_for_utxo};
use crate::vote::{generate_vote_file};
use crate::wallet::Wallet;
use crate::governance::governance::{Network, Governance, Constitution};
use crate::transaction::Transaction;

pub async fn constitution_update(mut tx:Transaction ,wallet:&Wallet, config:&Config, address:String)-> String{
    let mut govaction = Constitution::new(Network::Sancho, 0, String::from("https://shorturl.at/xMS15"), String::from("3d2a9d15382c14f5ca260a2f5bfb645fe148bfe10c1d0e1d305b7b1393e2bd97"), "https://shorturl.at/asIJ6".to_string(), String::from("./gov-actions/constitution.txt")  , String::from("./gov-actions/constitution.action"), String::from(""));
    let consti = govaction.create_constitution().await;
    generic_result_check(consti, String::from("Constitution file creation"));
    let status = govaction.create_action(wallet, &config.network);
    status_check(status, String::from("Governance action generation"));
    let build_status = tx.build_tx(wallet,String::from("--proposal-file"), config, String::from("./gov-actions/constitution.action"), address.clone());
    status_check(build_status, String::from("Governance action tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Governance action txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx(wallet);
    status_check(sign_status, String::from("Governance action tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(&config);
    status_check(submit_status, String::from("Governance action tx submission"));
    set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id.clone(), address).await;
    println!("Governance action transaction confirmed");
    return id;
}

pub async fn drep_register(mut tx:Transaction ,wallet:&Wallet, config:&Config, address:String){

    let build_status = tx.build_tx(wallet, String::from("--certificate-file"), config, String::from("./wallet/drep.cert"), address.clone());
    status_check(build_status, String::from("Drep registration tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Drep registration txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx(wallet);
    status_check(sign_status, String::from("Drep registration tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(&config);
    match submit_status{
        Ok(status) => {
            if status.success(){
                set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id, address).await
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

pub async fn vote_proposal(mut vote_tx:Transaction, action_id:String, wallet:&Wallet, config:&Config, address:String){
    generate_vote_file(String::from("yes"), String::from(action_id.trim()), String::from("./vote/vote"), String::from("0"), wallet).expect("Vote file generation unsuccessful");
    let build_status = vote_tx.build_tx(wallet, String::from("--vote-file"), config, String::from("./vote/vote"), address.clone());
    status_check(build_status, String::from("Voting tx build file generation"));
    let tx_id = vote_tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Vote txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = vote_tx.sign_tx_keys(vec![wallet.get_drep_file().get_private_path(), wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Vote tx Sign"));
    //submit the transaction
    let submit_status = vote_tx.submit_tx(&config);
    match submit_status{
        Ok(status) => {
            if status.success(){
                set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id, address).await
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

pub async fn delegate_drep(mut tx:Transaction ,holder_wallet:&Wallet, drep_wallet:&Wallet, config:&Config, address:String){
    drep_wallet.get_drep_file().delegate_to_me(holder_wallet.get_skey_file().get_public_path(), String::from("./wallet/deleg.cert")).expect("Delegation certificate creation not successful");
    let build_status = tx.build_tx(holder_wallet, String::from("--certificate-file"), config, String::from("./wallet/deleg.cert"), address.clone());
    status_check(build_status, String::from("Drep delegation tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Drep delegation txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx_keys(vec![holder_wallet.get_skey_file().get_private_path(), holder_wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Drep delegation tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(&config);
    match submit_status{
        Ok(status) => {
            if status.success(){
                set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id, address).await
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

pub async fn stake_reg_tx(mut tx:Transaction ,holder_wallet:&Wallet, config:&Config, address:String){
    let cert_status = holder_wallet.get_skey_file().gen_cert(200000);
    status_check(cert_status, String::from("Stake registration tx build file generation"));
    let build_status = tx.build_tx(holder_wallet, String::from("--certificate-file"), config, holder_wallet.get_skey_file().get_cert(), address.clone());
    status_check(build_status, String::from("Stake registration tx build file generation"));
    let tx_id = tx.get_tx_id();
    generic_result_check(tx_id.clone(), String::from("Stake registration txId construction"));
    let id = tx_id.unwrap();
    println!("Transaction id = {}", id);
    let sign_status = tx.sign_tx_keys(vec![holder_wallet.get_skey_file().get_private_path(), holder_wallet.get_pkey_file().get_private_path()]);
    status_check(sign_status, String::from("Stake registration tx Sign"));
    //submit the transaction
    let submit_status = tx.submit_tx(&config);
    match submit_status{
        Ok(status) => {
            if status.success(){
                set_interval(tokio::time::Duration::from_secs(2), check_for_utxo, id, address).await
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