use std::process::Command;

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
}

pub struct DrepKeyFile{
    public:String,
    private:String,
    certificate:String
}

impl PaymentKeyFile{
    pub fn new(public:String, private:String)->Self{
        Self {public, private}
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
    }
}

impl StakeKeyFile{
    pub fn new(public:String, private:String)->Self{
        Self {public, private}
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
    }
}

impl DrepKeyFile{
    pub fn new(public:String, private:String, certificate:String)->Self{
        Self {public, private, certificate}
    }
    pub fn gen_cert(&self)->Result<std::process::ExitStatus, std::io::Error>{
        let mut cli = Command::new("cardano-cli");
        let keypaircmd = cli.args(&["conway", "governance", "drep", "registration-certificate", "--drep-verification-key-file", self.public.as_str()])
                    .args(&["--out-file", self.certificate.as_str()])
                    .args(&["--key-reg-deposit-amt", "1000"]);
        
        keypaircmd.status()
    }
    pub fn get_public_path(&self)->String{
        self.public.clone()
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
