use std::env;

enum Commands {
    Set,
    Get,
    Delete
}

struct Handler {
    pub command: Commands,
    pub name: String,
}

pub struct Config {
    pub handler: Handler
}

impl Config {
    pub fn new(command: Commands, name: String) -> Config {
        let handler = Handler { command, name };
        Config {handler}
    }

    pub fn build () -> Result<Config, String> {
        let mut args = env::args();

        let command = match args.next().unwrap().as_str() {
            "set" => Commands::Set,
            "get" => Commands::Get,
            "delete" => Commands::Delete,
            _ => return Err("Unknown command".to_string())
        };

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("No name found".to_string())
        };

        Ok(Config::new(command, name))
    }
}