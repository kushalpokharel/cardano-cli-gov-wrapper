
use reqwest::Response;

use crate::actions;
use crate::transaction::Transaction;
use crate::utils::{query_utxo, self};
use crate::config::Config;
use crate::wallet::Wallet;

#[derive(serde::Serialize, serde::Deserialize)]
struct Utxo{
    amount:serde_json::Value,
    txid:String,
    txin:String
}

pub async fn maintain_balance_from_account(rec_address:String, send_address:String, send_wallet:&Wallet, config:&Config, index:u32)-> Result<Vec<String>, String>{
    let utxos = query_utxo(rec_address.as_str(), config.network.as_str())?;
    let mut available_utxos:Vec<String> = vec![];
    let utxoopt = get_utxos(&utxos,1000000, &mut available_utxos);
    match utxoopt{
        Err(_x) =>{
            //need to load balance from main account.
            let utxo = send_balance(config, vec![rec_address.clone()], send_wallet, &send_address, index).await?;
            Ok(poll_for_utxo(utxo, rec_address).await)
        }
        Ok(_utxos)=>{
            Ok(available_utxos)
        }
    }
}

pub async fn send_balance(config:&Config, rec_addresses:Vec<String>, send_wallet:&Wallet, send_address:&String, index:u32)->Result<String,String>{
    let tx_out = rec_addresses.iter().map(|rec_address|{
        rec_address.to_owned()+"+"+&String::from("10000000")
    }).collect();
    
    let utxos = maintain_balance_from_faucet(send_address.clone(), config).await?; 
    let tx_raw_file = String::from("./files/transactions/ada_tx.raw")+&index.to_string();
    let tx_signed_file = String::from("./files/transactions/ada_tx.signed") + & index.to_string();
    let tx: Transaction = Transaction::new(config.network.clone(), utxos, tx_out, tx_raw_file , tx_signed_file );
    let txid = actions::send_ada(tx, &send_wallet, &config, send_address.clone()).await?;
    Ok(txid)
}

pub async fn maintain_balance_from_faucet(address:String, config:&Config)-> Result<Vec<String>,String>{
    let utxos = query_utxo(address.as_str(), config.network.as_str())?;
    let mut available_utxos:Vec<String> = vec![];
    let utxoopt = get_utxos(&utxos,100000000, &mut available_utxos);
    match utxoopt{
        Err(_) =>{
            //need to load balance from faucet
            let resp = fetch_balance(config, address.clone()).await;
            Ok(check_fetch_result(resp, address).await)
        }
        Ok(_)=>{
            Ok(available_utxos)
        }
    }
}

async fn check_fetch_result(resp:Result<Response, String>, address:String)->Vec<String>{
    match resp{
        Ok(response) => {
            if response.status().is_success(){
                let _status = response.status().to_string();
                let json_response = response.text().await.unwrap();
                //parse the json_response
                let result: Result<Utxo, serde_json::Error> = serde_json::from_str(&json_response);
                // println!("Ada fetch request successful with status {} and response {}", status, json_response);
                get_recent_utxo(result, address).await
            }
            else{
                println!("Ada fetch request with status {} and response {}", response.status(), response.text().await.unwrap());
                std::process::exit(0);
            }
            
        },
        Err(e) => {
            println!("Failed Ada Fetch {}",e);
            std::process::exit(0);
        },
    }
}

async fn get_recent_utxo(result:Result<Utxo, serde_json::Error>, address:String)->Vec<String>{
    match result{
        Ok(utxo) => {
            println!("Faucet load successful with transaction id {}",utxo.txid);
            poll_for_utxo(utxo.txid, address).await
        },
        Err(err) => {
            println!("Failed to decode utxo from the response json {}", err);
            std::process::exit(0);
        },
    }
}

async fn poll_for_utxo(tx_id:String, address:String)->Vec<String>{
    let recent_utxo = utils::set_interval(tokio::time::Duration::from_secs(2) ,utils::check_for_utxo, tx_id, address ).await;
    vec![recent_utxo]
}

fn get_utxos<'a>(json:&serde_json::Value, balance:i64, utxo_list:&'a mut Vec<String>)->Result<&'a Vec<String>, String>{
    if balance<=0{
        return Ok(utxo_list);
    }
    match json.as_object(){
        Some(obj) => {
            if let Some((first_key, next)) = obj.iter().next(){
                let val = json[first_key]["value"]["lovelace"].as_i64().unwrap_or(0);
                utxo_list.push(first_key.clone());
                get_utxos(next, balance-val, utxo_list)
            }
            else{
                Err(String::from("Further json value not present: Not sufficient utxo"))
            }
        },
        None => Err(String::from("Couldn't parse the json: Not sufficient utxo")),
    }
}

async fn fetch_balance(config:&Config, address:String)->Result<reqwest::Response, String>{
    let url = config.faucet_url.clone()+&std::fmt::format(format_args!("?action=funds&address={}&api_key={}",address, config.api_key));
    reqwest::get(url)
    .await
    .map_err(|err|{
        err.to_string()
    })
}

