use std::{process::Command, str::from_utf8};
use crate::governance::governance::Network;

pub struct Transaction{
  tx_id:String,
  network:Network,
  tx_in:Vec<String>,
  tx_out:Vec<String>,
  change_address:String,
  tx_raw_file:String,
  tx_signed_file:String,
  governance_action_file:String
}

impl Transaction{
  pub fn new (network:Network, tx_in:Vec<String>, tx_id:String, tx_out:Vec<String> ,change_address:String, tx_raw_file:String, tx_signed_file:String, governance_action_file:String) -> Self { 
    Self {
      network,
      tx_id,
      tx_in,
      tx_out,
      change_address,
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
  pub fn get_tx_id(&self)->String{
    let mut docker = Command::new("docker");
    let cardano_cli_command = docker
        .args(&["exec", "cardano-node"]) // Replace with your Cardano node container name or ID
        .arg("cardano-cli")              // Replace with the actual name of the cardano-cli executable inside the container
        .arg("transaction")              // Replace with the cardano-cli subcommand and arguments you want to run
        .arg("txid")
        .arg("--tx-body-file")
        .arg(self.tx_raw_file.clone()); // Replace with the path to your transaction file

    match cardano_cli_command.status() {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Transaction submitted successfully!");
                from_utf8(&cardano_cli_command.output().unwrap().stdout).unwrap().to_string()

            } else {
                eprintln!("Transaction submission failed.");
                "".to_string()
            }
        }
        Err(err) => {
            eprintln!("Error running cardano-cli: {:?}", err);
            "".to_string()
        }
    }
  }
  pub fn set_governance_action_file(&mut self, file:String){
    self.governance_action_file = file;
  }

  pub fn sign_tx(&mut self)->bool {
    let mut docker = Command::new("docker");
    let network = match self.network{
      Network::Preview => ["--testnet-magic", "1"],
      Network::Preprod => ["--testnet-magic", "2"],
      Network::Sancho => ["--testnet-magic", "4"],
      Network::Mainnet => ["--mainnet", ""],
    };
    let inputs:Vec<_> = self.tx_in.iter().flat_map(|input|{
      ["--tx-in", input]
    }).collect();
    let cardano_cli_command = docker
        .args(&["exec", "cardano-node", "cardano-cli", "transaction", "sign", "--tx-body-file", "tx.raw"])
        .args(&network)
        .args(&["--signing-key-file", "payment.skey", "--signing-key-file", "drep.skey"])
        .arg("--out-file")
        .arg(self.tx_signed_file.clone()); // Replace with the path to your transaction file

    match cardano_cli_command.status() {
      Ok(exit_status) => {
          if exit_status.success() {
              println!("Transaction signed successfully!");
              true

          } else {
              eprintln!("Transaction signing failed.");
              false
          }
      }
      Err(err) => {
          eprintln!("Error running cardano-cli: {:?}", err);
          false
      }
    }
  }

  pub fn submit_tx(&mut self)->String{
    let mut docker = Command::new("docker");
    let cardano_cli_command = docker
        .args(&["exec", "cardano-node"]) // Replace with your Cardano node container name or ID
        .args(&["cardano-cli", "transaction", "submit", "--socket-path", "node.socket", "--testnet-magic", "4"])              // Replace with the actual name of the cardano-cli executable inside the container
        .arg("--tx-file")
        .arg(self.tx_signed_file.clone()); // Replace with the path to your transaction file

    match cardano_cli_command.status() {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Transaction submitted successfully!");
                from_utf8(&cardano_cli_command.output().unwrap().stdout).unwrap().to_string()

            } else {
                eprintln!("Transaction submission failed.");
                "".to_string()
            }
        }
        Err(err) => {
            eprintln!("Error running cardano-cli: {:?}", err);
            "".to_string()
        }
    }
  }

  pub fn build_tx(&self)->bool{
    let mut docker = Command::new("docker");
    let network = match self.network{
      Network::Preview => ["--testnet-magic", "1"],
      Network::Preprod => ["--testnet-magic", "2"],
      Network::Sancho => ["--testnet-magic", "4"],
      Network::Mainnet => ["--mainnet", ""],
    };
    let inputs:Vec<_> = self.tx_in.iter().flat_map(|input|{
      ["--tx-in", input]
    }).collect();
    let cardano_cli_command = docker
        .args(&["exec", "cardano-node", "cardano-cli", "transaction", "build", "--socket-path", "node.socket", "--conway-era"])
        .args(&network)
        .args(inputs)
        .args(&["--change-address",self.change_address.as_str(), "--constitution-file", self.governance_action_file.as_str()])
        .args(&["--witness-override", "2", "--out-file"])
        .arg(self.tx_raw_file.clone()); // Replace with the path to your transaction file

    match cardano_cli_command.status() {
      Ok(exit_status) => {
          if exit_status.success() {
              println!("Transaction built successfully!");
              true

          } else {
              eprintln!("Transaction built failed.");
              false
          }
      }
      Err(err) => {
          eprintln!("Error running cardano-cli: {:?}", err);
          false
      }
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
        change_address:String::new(),
        tx_raw_file:String::new(),
        tx_signed_file:String::new(),
        governance_action_file:String::new()
      }
  }
}