
pub mod governance{
    use std::process::Command;


  pub enum Network{
    Mainnet, Preview, Preprod, Sancho
  }
  pub struct Governance{

      network_id: Network,
      governance_action_deposit: u64,
      stake_verification_key_file: String,
      proposal_url: String,
      anchor_data_hash: String,
      constitution_url: String,
      constitution: String,
      out_file: String,
      action:String
  }

  impl Governance{
    pub fn new(network_id:Network, governance_action_deposit:u64, stake_verification_key_file:String, proposal_url:String, anchor_data_hash:String, constitution_url:String, constitution:String, out_file:String, action:String)->Self{
      Self{
        network_id,
        governance_action_deposit,
        stake_verification_key_file,
        proposal_url,
        anchor_data_hash,
        constitution_url,
        constitution,
        out_file,
        action
      }
    }
    pub fn create_action(&self)->bool{
      let mut docker = Command::new("docker");
      let network = match self.network_id{
        Network::Preview => ["--testnet"],
        Network::Preprod => ["--testnet"],
        Network::Sancho => ["--testnet"],
        Network::Mainnet => ["--mainnet"],
      };

      let cardano_cli_command = docker
          .args(&["exec", "cardano-node", "cardano-cli", "conway", "governance", "action", (self.action).as_str()])
          .args(&network)
          .args(&["--governance-action-deposit", self.governance_action_deposit.to_string().as_str(), "--proposal-url", self.proposal_url.as_str()])
          .args(&["--stake-verification-key-file",self.stake_verification_key_file.as_str(), "--anchor-data-hash", self.anchor_data_hash.as_str()])
          .args(&["--constitution-url", self.constitution_url.as_str(), "--constitution" , self.constitution.as_str(), "--out-file", (self.out_file.as_str())]);

      match cardano_cli_command.status() {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Governance action successfully built!");
                true

            } else {
                eprintln!("Governance action not successful.");
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
}