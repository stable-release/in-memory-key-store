use std::env::{self, Args};

pub struct Config {

}

impl Config {
    fn new(_args: Args) -> Config {
        Config {}   
    }

    pub fn build () -> Result<Config, String> {
        let args = env::args();

        Ok(Config::new(args))
    }
}