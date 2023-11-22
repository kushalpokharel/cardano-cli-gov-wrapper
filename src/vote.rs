use crate::wallet::Wallet;

pub fn generate_vote_file(vote:String, gov_action_id:String, out_file:String, action_idx:String, wallet:&Wallet)->Result<std::process::ExitStatus, std::io::Error>{
    let mut cli = std::process::Command::new("cardano-cli");
    let vote_option = String::from("--")+&vote;
    let cardano_cli_command = cli
        .args(["conway", "governance", "vote", "create"])
        .args([vote_option.as_str(), "--governance-action-tx-id", gov_action_id.as_str(), "--governance-action-index", action_idx.as_str()])
        .args(["--drep-verification-key-file",wallet.get_drep_file().get_public_path().as_str(), "--out-file", out_file.as_str() ]);

    cardano_cli_command.status()

}