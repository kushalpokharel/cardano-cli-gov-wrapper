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

pub fn load_file_contents(path:&String)->Option<String>{
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();

    println!("{}", path);
    let file = file.read_to_string(&mut buf);
    println!("{}", path);

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
    let config = bind_result(env::var("NETWORK"), |network|{
        Some(Config::new(network))
    });
    match config{
        None=> {
            println!("Config not set up properly");
            std::process::exit(0)
        },
        Some(conf)=>{
            return conf;
        }
    }
}

pub fn query_utxo(address:&String, network:&String)-> Option<String>{
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
                let utxo = extract_json(stdout_str);
                utxo
            } else {
                eprintln!("Error executing command: {:?}", output.status);
                None
            }
        }
        Err(_) => None,
    }
}

fn extract_json(json_string:String)->Option<String>{
    let parsed_json: serde_json::Result<Value> = serde_json::from_str(&json_string);
    match parsed_json {
        Ok(json) => {
            // Extract the first key from the JSON object
            if let Some((first_key, _)) = json.as_object()?.iter().next() {
                return Some(first_key.to_string());
            }
            else{
                return None;
            }
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
            None
        }
    }


}