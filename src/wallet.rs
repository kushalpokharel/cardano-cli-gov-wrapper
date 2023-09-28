use std::{process::{ExitStatus, Command}, io::Error};

use crate::{KeyPair, keypairs::{PaymentKeyFile, StakeKeyFile, DrepKeyFile}};
pub struct Wallet{
    pkey_file:PaymentKeyFile,
    skey_file:StakeKeyFile,
    dkey_file:DrepKeyFile,
    p_addrfile:String
}

impl Wallet{
    pub fn new(pkey_file:PaymentKeyFile, skey_file:StakeKeyFile,dkey_file:DrepKeyFile,p_addrfile:String)->Self{
        Wallet{pkey_file, skey_file, dkey_file, p_addrfile}
    }
    pub fn gen_addr(&self, network:String)->Result<ExitStatus, Error>{
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["address", "build", "--payment-verification-key-file", self.pkey_file.get_public_path().as_str()])
                    .args(&["--stake-verification-key-file", self.skey_file.get_public_path().as_str(), "--testnet-magic", network.as_str(), "--out-file", self.p_addrfile.as_str()]);
        
        keypaircmd.status()
    }
    
    pub fn get_skey_file(&self)->&StakeKeyFile{
        &self.skey_file
    }
    pub fn get_pkey_file(&self)->&PaymentKeyFile{
        &self.pkey_file
    }
    pub fn get_paddr_file(&self)->&String{
        &self.p_addrfile
    }
    pub fn get_drep_file(&self)->&DrepKeyFile{
        &self.dkey_file
    }
}