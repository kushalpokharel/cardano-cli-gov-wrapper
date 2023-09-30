use std::{process::Command, str::from_utf8};
use crate::{governance::governance::Network, wallet::Wallet, utils::{query_utxo, load_env, load_file_contents}};

pub struct Transaction{
  tx_id:String,
  network:Network,
  tx_in:Vec<String>,
  tx_out:Vec<String>,
  tx_raw_file:String,
  tx_signed_file:String,
  governance_action_file:String
}

impl Transaction{
  pub fn new (network:Network, tx_in:Vec<String>, tx_id:String, tx_out:Vec<String> , tx_raw_file:String, tx_signed_file:String, governance_action_file:String) -> Self { 
    Self {
      network,
      tx_id,
      tx_in,
      tx_out,
      tx_raw_file,
      tx_signed_file,
      governance_action_file
    }
  }
  pub fn add_input(&mut self, input:String){
    self.tx_in.push(input);
  }
  pub fn add_output(&mut self, output:String){
    self.tx_out.push(output);
  }
  pub fn out_file(&mut self, filepath:String){
    self.tx_raw_file = filepath;
  }
  pub fn get_tx_id(&self)->Result<std::process::ExitStatus, std::io::Error>{
    let mut cmd = Command::new("cardano-cli");
    let cardano_cli_command = cmd
        .arg("transaction")              // Replace with the cardano-cli subcommand and arguments you want to run
        .arg("txid")
        .arg("--tx-body-file")
        .arg(self.tx_raw_file.clone()); // Replace with the path to your transaction file

    cardano_cli_command.status()
  }
  pub fn set_governance_action_file(&mut self, file:String){
    self.governance_action_file = file;
  }

  pub fn sign_tx(&mut self, wallet:&Wallet)->Result<std::process::ExitStatus, std::io::Error> {
    let mut cmd = Command::new("cardano-cli");
    let network = match self.network{
      Network::Preview => ["--testnet-magic", "1"],
      Network::Preprod => ["--testnet-magic", "2"],
      Network::Sancho => ["--testnet-magic", "4"],
      Network::Mainnet => ["--mainnet", ""],
    };
    let inputs:Vec<_> = self.tx_in.iter().flat_map(|input|{
      ["--tx-in", input]
    }).collect();
    let cardano_cli_command = cmd
        .args(&["transaction", "sign", "--tx-body-file", self.tx_raw_file.as_str()])
        .args(&network)
        .args(&["--signing-key-file", wallet.get_pkey_file().get_private_path().as_str(), "--signing-key-file", wallet.get_drep_file().get_private_path().as_str()])
        .arg("--out-file")
        .arg(self.tx_signed_file.clone()); // Replace with the path to your transaction file

    cardano_cli_command.status()
  }

  pub fn submit_tx(&mut self)->Result<std::process::ExitStatus, std::io::Error>{
    let mut cmd = Command::new("cardano-cli");
    let cardano_cli_command = cmd
        .args(&["transaction", "submit", "--socket-path", "/home/kushal/.cardano/sancho/node.socket", "--testnet-magic", "4"])              // Replace with the actual name of the cardano-cli executable inside the container
        .arg("--tx-file")
        .arg(self.tx_signed_file.clone()); // Replace with the path to your transaction file

    cardano_cli_command.status()
  }

  pub fn build_tx(&self, wallet:&Wallet, action_option:String, network:&String, action_file:String)->Option<()>{
    let mut cmd = Command::new("cardano-cli");
    match load_file_contents(wallet.get_paddr_file()){
        Some(address) => {
          match query_utxo(&address, network){
            Some(utxo) => {
              println!("{}", utxo);
              let inputs:Vec<_> = self.tx_in.iter().flat_map(|input|{
                ["--tx-in", input]
              }).collect();
              let cardano_cli_command = cmd
                  .args(&["transaction", "build", "--socket-path", "/home/kushal/.cardano/sancho/node.socket", "--conway-era"])
                  .args(&["--testnet-magic",network.as_str()])
                  .args(&["--tx-in", utxo.as_str()])
                  .args(&["--change-address",address.as_str(), action_option.as_str(), action_file.as_str()])
                  .args(&["--witness-override", "2", "--out-file"])
                  .arg(self.tx_raw_file.clone()); // Replace with the path to your transaction file
          
              match cardano_cli_command.status(){
                Ok(_)=>{
                  Some(())
                },
                Err(_)=>{
          
                  None
                }
              }
            },
            None => None
          }
        },
        None => None,
    }
  }
}

impl Default for Transaction{
  fn default() -> Self {
      Transaction {
        tx_id:String::new(),
        network:Network::Mainnet,
        tx_in:Vec::new(),
        tx_out:Vec::new(),
        tx_raw_file:String::new(),
        tx_signed_file:String::new(),
        governance_action_file:String::new()
      }
  }
}