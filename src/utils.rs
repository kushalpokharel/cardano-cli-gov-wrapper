use dotenv::dotenv;
use std::{env, fs::File, io::{Read, self, Write, Error}, process::{Command, Stdio}};
use crate::config::Config;
use serde_json::Value; 

pub fn bind_result<T, E, U, F>(option: Result<T, E>, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>,
{
    match option {
        Ok(value) => f(value),
        Err(_) => None,
    }
}

// pub fn bind_results<T, E, U, F>(option: Result<T, E>, f: F) -> Result<T,E>
// where
//     F: FnOnce(T) -> Result<T,E>,
// {
//     match option {
//         Ok(value) => f(value),
//         Err(_) => Err(),
//     }
// }

pub fn load_file_contents(path:&String)->Option<String>{
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();

    let file = file.read_to_string(&mut buf);

    match file{
        Ok(_)=> Some(buf),
        Err(_) => None
    }
}

pub fn status_check(status:Result<std::process::ExitStatus, std::io::Error>, function:String){
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
        Err(err) => {
            eprintln!("{} not successful", function);
            std::process::exit(0);
        }
    }
}

pub fn load_env()->Config{
    dotenv().ok();
    let config = env::var("NETWORK").and_then(|network|{
        env::var("NODE_SOCKET_PATH").and_then(|socket_path|{
            env::var("API_KEY").and_then(|api_key|{
                match env::var("FAUCET_URL"){
                    Ok(faucet_url) => Ok(Config::from(network, socket_path, faucet_url, api_key)),
                    Err(_) => Ok(Config::new(network, socket_path, api_key)),
                }
            })
        })
    });
    match config{
        Err(_)=> {
            println!("Config not set up properly");
            std::process::exit(0)
        },
        Ok(conf)=>{
            return conf;
        }
    }
}

pub fn query_utxo(address:&String, network:&String)-> Result<Value, String>{
    let mut cmd = Command::new("cardano-cli");
    
    let cardano_cli_command = cmd
        .args(&["query", "utxo", "--address", address.as_str(), "--testnet-magic", network.as_str()])
        .args(&["--out-file", "/dev/stdout",  "--socket-path", "/home/kushal/.cardano/sancho/node.socket"])
        .stdout(Stdio::piped())
        .spawn();
    match cardano_cli_command{
        Ok(mut child) => {
            let output = child.wait_with_output().expect("Failed to wait for command");

            if output.status.success() {
                // Extract the stdout as a String
                let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                let json_utxo = extract_json(stdout_str);
                json_utxo
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

pub async fn set_interval<F, P>(interval: tokio::time::Duration, mut task: F, txid:P, address:P)
where
    F: FnMut(P, P) ->bool + Send + 'static,
    P: PartialEq + Send + Clone+ 'static,
{
    loop {
        // Perform the task
        let res = task(txid.clone(), address.clone());
        if res{
            break;
        }

        // Sleep for the specified interval
        tokio::time::sleep(interval).await;
    }
}

pub fn check_for_utxo(txid:String, address:String)-> bool{
    let json_value = query_utxo(&address, &String::from("4"));
    match json_value{
        Ok(value) => {
            let check = recurse_json(&value, txid);
            check
        },
        Err(err) => {
            println!("Error while querying utxos");
            std::process::exit(0);
        },
    }
}

pub fn recurse_json(json_value:&Value, txid:String)->bool{
    match json_value.as_object(){
        Some(obj) => {
            if let Some((first_key, next)) = obj.iter().next(){
                let result = recurse_json(next, txid.clone());
                if (*first_key).contains(&txid){
                    return true;
                }
                else{
                    return result || false;
                }
            }
            else{
                return false;
            }
        },
        None => return false,
    }
}