## Gov Action Loader 
This repository helps in creation of transactions specific to Conway era that are available in the cardano-cli as of now. Wallet is setup with fresh keys and various transactions like Create-Constitution proposal creation ,dRep registration, dRep delegation and Voting are made. Transaction ids for each transaction is returned.


## Run the code 
- ### Prerequisite 
    - You should have sancho-node runnning and cardano-cli available.
- **Install rust compiler**
https://doc.rust-lang.org/book/ch01-01-installation.html
- Clone the repository
- Add .env file with all the keys from .env.example
- Run `cargo run` in the repository to run the program


### The code does following in order
- Generate key-pairs for ada-holder and drep
- Register drep 
- Propose new Constitution
- Register ada_holder's stake_key 
- Delegate ada_holder to the drep 
- Drep vote to the new gov-actions

You can see if the vote to the governance action was successful using this command in your shell:
- `cardano-cli query ledger-state --testnet-magic 4 | jq -r --arg key_to_match ${your_gov_action_id} '.stateBefore.esLState.utxoState.ppups.gov.curGovActionsState | select(.[$key_to_match] != null) | {($key_to_match): .[$key_to_match]}'`