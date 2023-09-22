pub struct Config{
    pub network:String
}

impl Config {
    pub fn new(magic:String)->Self{
        Self{network:magic }
    }
}