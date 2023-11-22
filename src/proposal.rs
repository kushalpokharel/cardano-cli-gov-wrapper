
use crate::balance::{maintain_balance_from_faucet, send_balance, maintain_balance_from_account};
use crate::config::Config;
use crate::{get_address, setup, actions};
use crate::wallet::Wallet;

use crate::transaction::Transaction;

pub async fn make_proposal(config:&std::sync::Arc<Config>, number_of_proposals:u32)->Result<(Vec<String>,Vec<String>), String>{
    let minimum = std::cmp::min(number_of_proposals , 5);
    let mut ada_holder_wallets:Vec<Wallet>= vec![];
    let mut ada_holder_addresses:Vec<String> = vec![];
    for n in 0..minimum{
        let ada_holder_wallet = setup::setup_wallet(String::from("ada_holder")+&n.to_string(),&config)?;
        let holder_address = get_address(&ada_holder_wallet);
        ada_holder_wallets.push(ada_holder_wallet);
        ada_holder_addresses.push(holder_address);
    }

    maintain_balance_from_faucet(ada_holder_addresses[0].clone(), &config).await?;
    send_balance(config, ada_holder_addresses[1..5].to_owned(), &ada_holder_wallets[0], &ada_holder_addresses[0], 9).await?;

    let ada_holder_addresses = std::sync::Arc::new(ada_holder_addresses);
    let ada_holder_wallets = std::sync::Arc::new(ada_holder_wallets);
    let mut handles = vec![];
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(minimum as usize));
    let wallets_idx = (0..5).collect();
    let available_wallets:std::sync::Arc<tokio::sync::Mutex<Vec<i32>>> = std::sync::Arc::new(tokio::sync::Mutex::new(wallets_idx));
    for i in 0..number_of_proposals{
        let wallets = std::sync::Arc::clone(&ada_holder_wallets);
        let addresses = std::sync::Arc::clone(&ada_holder_addresses);
        let config = std::sync::Arc::clone(&config);
        let sem_clone = std::sync::Arc::clone(&semaphore);
        let available_wallets = std::sync::Arc::clone(&available_wallets);
        let handle = tokio::spawn(async move{
            let permit = sem_clone.acquire_owned().await.unwrap();
            let mut wallets_idx = available_wallets.lock().await;
            let idx = wallets_idx.pop().expect("Problem while accessing available wallets in thread");
            std::mem::drop(wallets_idx);
            let id = idx as usize;
            let tx = constitution_transaction(addresses[id].clone(), &wallets[id], &wallets[0], addresses[0].clone(), &*config, i).await;
            let mut wallets_idx = available_wallets.lock().await;
            wallets_idx.push(idx);
            drop(permit);
            tx
        });
        handles.push(handle);
    }
    let results = futures::future::try_join_all(handles).await;
    match results {
        Ok(results) => {
            let (success, failed): (Vec<Result<String, String>>, Vec<Result<String, String>>) =
                results.into_iter().partition(Result::is_ok);
            let success: Vec<String> = success.into_iter().map(Result::unwrap).collect();
            let failed: Vec<String> = failed.into_iter().map(Result::unwrap_err).collect();
            Ok((success,failed))
        }
        Err(_) => Err(String::from("Threads cancelled")) // Handle task cancelation
    }

}

pub async fn constitution_transaction(drep_address:String, drep_wallet:&Wallet, fund_wallet:&Wallet, fund_address:String, config:&Config, i:u32)->Result<String, String>{
    let tx_raw_file = String::from("./files/transactions/const_tx.raw")+&i.to_string();
    let tx_signed_file = String::from("./files/transactions/const_tx.signed") +&i.to_string();
    let utxos = maintain_balance_from_account(drep_address.clone(), fund_address, fund_wallet, &config, i).await?; 
    let tx: Transaction = Transaction::new(config.network.clone(), utxos, vec![], tx_raw_file, tx_signed_file);
    let out_file = String::from("./files/gov-actions/constitution.action")+&i.to_string();  
    let txid = actions::constitution_update(tx, &drep_wallet, &config, drep_address.clone(), String::from("https://shorturl.at/xMS15"), String::from("c2cdc3b47d433194a947042b305a1951391b96d5a1ae4de2e0f37fb20eea3560"), String::from("./files/gov-actions/constitution.txt"), "https://shorturl.at/asIJ6".to_string(), out_file, i).await?;
    Ok(txid)
}
