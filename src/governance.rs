
pub mod governance{
    use std::{process::{Command, ExitStatus}, io::{Error, Read, Write}, fs::{self, File}};

    use crate::wallet::Wallet;
    
    use reqwest;

  pub enum Network{
    Mainnet, Preview, Preprod, Sancho
  }

  pub trait Governance{
    fn create_action(&self, wallet:&Wallet, network:&String)->Result<ExitStatus, Error>;
  }
  pub struct Constitution{

      network_id: Network,
      governance_action_deposit: u64,
      proposal_url: String,
      proposal_hash: String,
      constitution_url: String,
      constitution_file: String,
      constitution:String,
      out_file: String,
  }

  impl Constitution{
    pub fn new(network_id:Network, governance_action_deposit:u64, proposal_url:String, proposal_hash:String, constitution_url:String, constitution_file:String, out_file:String, constitution:String)->Self{
      Self{
        network_id,
        governance_action_deposit,
        proposal_url,
        proposal_hash,
        constitution_url,
        constitution_file,
        constitution,
        out_file
      }
    }
    pub async fn create_constitution(& mut self)->Result<String, Box<dyn std::error::Error>>{
      match fs::metadata(&self.constitution_file){
        Ok(_) => {
          let mut file = File::open("./files/gov-actions/constitution.txt").unwrap();
          let mut buf = String::new();
          let file = file.read_to_string(&mut buf);
          match file{
            Ok(_) => {
              self.constitution = buf.clone();
              Ok(buf)
            },
            Err(x) => Err(Box::new(x)),
        }
        }
        Err(_) => {
          let res = reqwest::get(String::from("https://raw.githubusercontent.com/input-output-hk/sanchonet/master/README.md")).await;
          match res{
            Ok(x)=> {
              match x.text().await{
                Ok(x)=> {
                  let file  = File::create("./files/gov-actions/constitution.txt");
                  match file{
                    Ok(mut f)=> match f.write_all(x.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => println!("Error writing to constitution file"),
                    },
                    Err(_)=> println!("Error writing to constitution file")
                  };
                  self.constitution = x.clone();
                  Ok(x)
                }
                Err(e)=> Err(Box::new(e))
              }
            }
            Err(x) => Err(Box::new(x))
          }
          
        }
    }
    }
  }
  impl Governance for Constitution{
    fn create_action(&self, wallet:&Wallet, _network:&String)->Result<ExitStatus, Error>{
      let mut cli = Command::new("cardano-cli");

      let cardano_cli_command = cli
          .args(&["conway", "governance", "action", "create-constitution"])
          .args(&["--testnet", "--governance-action-deposit", self.governance_action_deposit.to_string().as_str(), "--proposal-url", self.proposal_url.as_str()])
          .args(&["--stake-verification-key-file",wallet.get_skey_file().get_public_path().as_str(), "--proposal-hash", self.proposal_hash.as_str()])
          .args(&["--constitution-url", self.constitution_url.as_str(), "--constitution-file" , self.constitution_file.as_str(), "--out-file", (self.out_file.as_str())]);

      cardano_cli_command.status()
    }
  }
}