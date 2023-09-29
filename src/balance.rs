use crate::transaction::Transaction;
use crate::utils::{status_check, query_utxo, self};
use crate::config::{Config};

#[derive(serde::Serialize, serde::Deserialize)]
struct Utxo{
    amount:serde_json::Value,
    txid:String,
    txin:String
}

pub async fn maintain_balance(address:String, config:&Config)-> Vec<String>{
    let utxosres = query_utxo(&address, &config.network);
    let utxos = match utxosres{
        Err(_) =>{
            println!("Could not query utxos realted to this address {}", address);
            std::process::exit(0);
        }
        Ok(x) => x
    };
    let mut available_utxos:Vec<String> = vec![];
    let utxoopt = get_utxos(&utxos,100000000, &mut available_utxos);
    match utxoopt{
        None =>{
            //need to load balance from faucet
            let resp = fetch_balance(&config, address.clone()).await;
            match resp{
                Ok(response) => {
                    if response.status().is_success(){
                        let status = response.status().to_string();
                        let json_response = response.text().await.unwrap();
                        //parse the json_response
                        let result: Result<Utxo, serde_json::Error> = serde_json::from_str(&json_response);
                        // println!("Ada fetch request successful with status {} and response {}", status, json_response);
                        match result{
                            Ok(utxo) => {
                                println!("{}",utxo.txid);
                                utils::set_interval(tokio::time::Duration::from_secs(2) ,utils::check_for_utxo, utxo.txid.clone(), address ).await;
                                vec![utxo.txid.clone()+"#0"]
                            },
                            Err(err) => {
                                println!("Failed to decode utxo from the response json {}", err.to_string());
                                std::process::exit(0);
                            },
                        }
                    }
                    else{
                        println!("Ada fetch request with status {} and response {}", response.status().to_string(), response.text().await.unwrap());
                        std::process::exit(0);
                    }
                    
                },
                Err(e) => {
                    println!("Failed Ada Fetch {}",e);
                    std::process::exit(0);
                },
            }
        }
        Some(utxos)=>{
            println!("Sufficient balance in the wallet {:?}", utxos );
            available_utxos
        }
    }
}

fn get_utxos<'a>(json:&serde_json::Value, balance:i64, utxo_list:&'a mut Vec<String>)->Option<&'a Vec<String>>{
    if balance<=0{
        return Some(utxo_list);
    }
    match json.as_object(){
        Some(obj) => {
            if let Some((first_key, next)) = obj.iter().next(){
                let val = match json[first_key]["value"]["lovelace"].as_i64(){
                    Some(x) => x,
                    None => 0,
                };
                utxo_list.push(first_key.clone());
                let res = get_utxos(next, balance-val, utxo_list);
                match res{
                    None => None,
                    Some(utxos) => Some(utxos)
                }
            }
            else{
                None
            }
        },
        None => None,
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

