use dotenv::dotenv;
use rand::Rng;
use std::{env, fs::File, io::Read, process::{Command, Stdio}};
use crate::config::Config;
use serde_json::Value; 


pub fn gen_random_hex(size:u32)->String{
    let mut rng = rand::thread_rng();
    let hex_string = (0..size)
        .map(|_| format!("{:X}", rng.gen::<u8>()))
        .collect();
    hex_string
}

pub fn load_file_contents(path:&String)->Result<String, String>{
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();

    let file = file.read_to_string(&mut buf);
    match file{
        Ok(_)=> Ok(buf),
        Err(err) => Err(err.to_string())
    }
}

pub fn status_check(status:Result<std::process::ExitStatus, std::io::Error>, function:String)->Result<String, String>{
    match status{
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(String::from("Successful execution"))
                
            } else {
                Err(function+&" unsuccessful".to_string())
            }
        }
        Err(err) => {
            Err("Error running cardano-cli: ".to_owned()+&err.to_string())
        }
    }
}

pub fn status_check_exit(status:Result<std::process::ExitStatus, std::io::Error>, function:String){
    match status{
        Ok(exit_status) => {
            if exit_status.success() {
                println!("{0} successful!", function );
                
                
            } else {
                eprintln!("{0} unsuccessful", function);
                std::process::exit(0)
            }
        }
        Err(err) => {
            eprintln!("Error running cardano-cli: {:?}", err);
            std::process::exit(0);
        }
    }
}

pub fn generic_result_check<T,E>(res:Result<T,E>, function:String){
    match res{
        Ok(_) => {
           println!("{} successful", function);
        }
        Err(_err) => {
            eprintln!("{} not successful", function);
            std::process::exit(0);
        }
    }
}

pub fn load_env()->Result<Config, String>{
    dotenv().ok();
    let config = env::var("NETWORK").and_then(|network|{
        env::var("NODE_SOCKET_PATH").and_then(|socket_path|{
            env::var("FAUCET_API_KEY").map(|api_key|{
                match env::var("FAUCET_URL"){
                    Ok(faucet_url) => Config::from(network, socket_path, faucet_url, api_key),
                    Err(_) => Config::new(network, socket_path, api_key),
                }
            })
        })
    });
    config.map_err(|err|{err.to_string()})
}

pub fn query_utxo(address:&str, network:&str)-> Result<Value, String>{
    let mut cmd = Command::new("cardano-cli");
    let out_file = String::from("./files/utxos/utxo") + &gen_random_hex(10);
    let cardano_cli_command = cmd
        .args(["query", "utxo", "--address", address, "--testnet-magic", network])
        .args(["--out-file", out_file.as_str(),  "--socket-path", "/home/kushal/.cardano/sancho/node.socket"]);
    match cardano_cli_command.status(){
        Ok(status) => {
            if status.success() {
                // Extract the stdout as a String
                let stdout_str = load_file_contents(&out_file).expect("Reading utxo file not successful");
                extract_json(stdout_str)
            } else {
                Err(String::from("Constitution not successful"))
            }
        }
        Err(x) => Err(x.to_string()),
    }
}

fn extract_json(json_string:String)->Result<Value, String>{
    let parsed_json: serde_json::Result<Value> = serde_json::from_str(&json_string);
    parsed_json.map_err(|err|{
        err.to_string()
    })

}

pub async fn set_interval<F, P>(interval: tokio::time::Duration, mut task: F, txid:P, address:P) -> P
where
    F: FnMut(P, P) ->Result<P, P> + Send + 'static,
    P: PartialEq + Send + Clone+ 'static,
{
    loop {
        // Perform the task
        let res = task(txid.clone(), address.clone());
        if let Ok(utxo) = res{
            return utxo;
        }

        // Sleep for the specified interval
        tokio::time::sleep(interval).await;
    }
}

pub fn check_for_utxo(txid:String, address:String)-> Result<String, String>{
    let json_value = query_utxo(address.as_str(), "4");
    match json_value{
        Ok(value) => {
            
            recurse_json(&value, txid)
        },
        Err(err) => {
            println!("Error while querying utxos,{}", err);
            std::process::exit(0);
        },
    }
}

fn recurse_json(json_value:&Value, txid:String)->Result<String, String>{
    match json_value.as_object(){
        Some(obj) => {
            for key in obj.keys(){
                if key.contains(txid.trim_end()){
                    return Ok(key.to_string());
                }
            }
            Err("Doesn't contain the txid".to_string())
        },
        None => Err("Can't parse json".to_string()),
    }
}