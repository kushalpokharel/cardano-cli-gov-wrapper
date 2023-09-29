use std::process::Command;

use crate::utils::load_file_contents;

pub trait KeyPair{
    fn gen_pair(&self)->Result<std::process::ExitStatus, std::io::Error>;
    fn gen_public(&self)->Result<std::process::ExitStatus, std::io::Error>;
    fn gen_addr(&self, network:String)-> Result<std::process::ExitStatus, std::io::Error>;
}
pub struct PaymentKeyFile{
    public:String,
    private:String,
}

pub struct StakeKeyFile{
    public:String,
    private:String,
    certificate:String
}

pub struct DrepKeyFile{
    public:String,
    private:String,
    certificate:String, 
    id:String,
}

impl PaymentKeyFile{
    pub fn new(public:String, private:String)->Self{
        Self {public, private}
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
    }
    pub fn get_private_path(&self)->String{
        self.private.clone()
    }
}

impl StakeKeyFile{
    pub fn new(public:String, private:String, certificate:String)->Self{
        Self {public, private, certificate}
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
    }
    pub fn get_private_path(&self)->String{
        self.private.clone()
    }
    pub fn gen_cert(&self, deposit:u32)->Result<std::process::ExitStatus, std::io::Error>{
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["stake-address", "registration-certificate", "--stake-verification-key-file", self.public.as_str()])
                    .args(&["--out-file", self.certificate.as_str()])
                    .args(&["--key-reg-deposit-amt", deposit.to_string().as_str()]);
        
        keypaircmd.status()
    }
    pub fn get_cert(&self)->String{
        self.certificate.clone()
    }
}

impl DrepKeyFile{
    pub fn new(public:String, private:String, certificate:String, id:String)->Self{
        Self {public, private, certificate, id}
    }
    pub fn gen_cert(&self)->Result<std::process::ExitStatus, std::io::Error>{
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["conway", "governance", "drep", "registration-certificate", "--drep-verification-key-file", self.public.as_str()])
                    .args(&["--out-file", self.certificate.as_str()])
                    .args(&["--key-reg-deposit-amt", "0"]);
        
        keypaircmd.status()
    }
    pub fn gen_id(&self)->Result<std::process::ExitStatus, std::io::Error>{
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["conway", "governance", "drep", "id", "--drep-verification-key-file", self.public.as_str()])
                    .args(&["--out-file", self.id.as_str()]);
        
        keypaircmd.status()
    }

    pub fn delegate_to_me(&self, stake_addr_file:String, path:String) -> Result<std::process::ExitStatus, std::io::Error>{
        let mut cli = Command::new("cardano-cli");
        match load_file_contents(&self.id){
            None => return Err(std::io::ErrorKind::NotFound.into()),
            Some(content)=> {
                println!("{}", content);
                let keypaircmd = cli.args(&["conway", "stake-address", "vote-delegation-certificate", "--stake-verification-key-file", stake_addr_file.as_str()])
                            .args(["--drep-key-hash", content.as_str()])
                            .args(&["--out-file", path.as_str()]);

                keypaircmd.status()
            }
        }
        
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
    }
    pub fn get_private_path(&self)->String{
        self.private.clone()
    }
}

impl KeyPair for PaymentKeyFile{
    fn gen_pair(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["address", "key-gen", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
        
    }

    fn gen_public(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["key", "verification-key", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
    }

    fn gen_addr(&self, network:String)-> Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["address", "build", "--payment-verification-key-file", self.private.as_str(), "--testnet-magic", network.as_str()]);
                    // .args(&["--out-file", self.private.as_str()]);
        
        keypaircmd.status()
    }
}

impl KeyPair for StakeKeyFile{
    fn gen_pair(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["stake-address", "key-gen", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
        
    }

    fn gen_public(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["key", "verification-key", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
    }
    fn gen_addr(&self, network:String)-> Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["stake-addresss", "build", "--stake-verification-key-file", self.private.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
    }
}


impl KeyPair for DrepKeyFile{
    fn gen_pair(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["conway", "governance", "drep", "key-gen", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
        
    }

    fn gen_public(&self)->Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["key", "verification-key", "--verification-key-file", self.public.as_str()])
                    .args(&["--signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
    }
    fn gen_addr(&self, network:String)-> Result<std::process::ExitStatus, std::io::Error> {
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["conway", "governance", "id", "--drep-verification-key-file", self.public.as_str()])
                    .args(&["--drep-signing-key-file", self.private.as_str()]);
        
        keypaircmd.status()
    }
}
