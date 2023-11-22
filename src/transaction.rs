use std::process::Command;
use crate::{wallet::Wallet, config::Config};

pub struct Transaction{
  network:String,
  tx_in:Vec<String>,
  tx_out:Vec<String>,
  tx_raw_file:String,
  tx_signed_file:String,
}

impl Transaction{
  pub fn new (network:String, tx_in:Vec<String>, tx_out:Vec<String>,tx_raw_file:String, tx_signed_file:String) -> Self { 
    Self {
      network,
      tx_in,
      tx_out,
      tx_raw_file,
      tx_signed_file
    }
  }
  pub fn _add_input_list(&mut self, input_list:&Vec<String>){
    self.tx_in.extend_from_slice(input_list)
  }
  pub fn get_tx_id(&self)->Result<String, String>{
    let mut cmd = Command::new("cardano-cli");
    let cardano_cli_command = cmd
        .arg("transaction")              // Replace with the cardano-cli subcommand and arguments you want to run
        .arg("txid")
        .arg("--tx-body-file")
        .arg(self.tx_raw_file.clone()); // Replace with the path to your transaction file

    // let status = cardano_cli_command.status();
    match cardano_cli_command.output(){
        Ok(output) => {
          let out_str = String::from_utf8_lossy(&output.stdout).to_string();
          Ok(out_str)
        },
        Err(_) => Err(String::from("Transaction id not loading"))
    }
  }

  pub fn sign_tx(&mut self, wallet:&Wallet)->Result<std::process::ExitStatus, std::io::Error> {
    let mut cmd = Command::new("cardano-cli");
    let cardano_cli_command = cmd
        .args(["transaction", "sign", "--tx-body-file", self.tx_raw_file.as_str()])
        .args(["--testnet-magic", self.network.as_str()])
        .args(["--signing-key-file", wallet.get_pkey_file().get_private_path().as_str()])
        .arg("--out-file")
        .arg(self.tx_signed_file.clone()); // Replace with the path to your transaction file

    cardano_cli_command.status()
  }

  pub fn sign_tx_keys(&mut self, keys_list:Vec<String>)->Result<std::process::ExitStatus, std::io::Error> {
    let mut cmd = Command::new("cardano-cli");
    let list:Vec<_> = keys_list.into_iter().flat_map(|key|{
      ["--signing-key-file".to_string(), key]
    }).collect();
    let cardano_cli_command = cmd
        .args(["transaction", "sign", "--tx-body-file", self.tx_raw_file.as_str()])
        .args(["--testnet-magic", self.network.as_str() ])
        .args(&list)
        .arg("--out-file")
        .arg(self.tx_signed_file.clone());

    cardano_cli_command.status()
  }

  pub fn submit_tx(&mut self, config:&Config)->Result<std::process::ExitStatus, std::io::Error>{
    let mut cmd = Command::new("cardano-cli");
    let cardano_cli_command = cmd
        .args(["transaction", "submit", "--socket-path", config.socket_path.as_str(), "--testnet-magic", self.network.as_str()]) 
        .arg("--tx-file")
        .arg(self.tx_signed_file.clone());

    cardano_cli_command.status()
  }

  pub fn build_tx(&self, _wallet:&Wallet, action_option:String, config:&Config, action_file:String, number_of_signs:String, address:String)->Result<std::process::ExitStatus, std::io::Error>{
    let mut cmd = Command::new("cardano-cli");
    let inputs:Vec<_> = self.tx_in.iter().flat_map(|input|{
      ["--tx-in", input]
    }).collect();
    let outputs:Vec<_> = self.tx_out.iter().flat_map(|output|{
      ["--tx-out", output]
    }).collect();
    let action = if action_option!="" {
      vec![action_option.as_str(), action_file.as_str()]
    }
    else{vec![]};

    let cardano_cli_command = cmd
        .args(["conway", "transaction", "build", "--socket-path", config.socket_path.as_str()])
        .args(["--testnet-magic",config.network.as_str()])
        .args(&inputs)
        .args(&outputs)
        .args(["--change-address",address.as_str()])
        .args(&action)
        .args(["--witness-override", number_of_signs.as_str(), "--out-file"])
        .arg(self.tx_raw_file.clone()); 
    cardano_cli_command.status()
  }
}

impl Default for Transaction{
  fn default() -> Self {
      Transaction {
        network:String::from("4"),
        tx_in:Vec::new(),
        tx_out:Vec::new(),
        tx_raw_file:String::new(),
        tx_signed_file:String::new()
      }
  }
}