pub struct Config{
    pub network:String,
    pub socket_path:String,
    pub faucet_url: String,
    pub api_key:String
}

impl Config {
    pub fn new(magic:String, socket_path:String, api_key:String)->Self{
        Self{network:magic, socket_path, faucet_url:String::from("https://faucet.sanchonet.world.dev.cardano.org/send-money"), api_key}
    }
    pub fn from(magic:String, socket_path:String, faucet_url:String, api_key:String)->Self{
        Self{network:magic, socket_path, faucet_url, api_key}
    }
}