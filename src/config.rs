use std::{
    collections::HashMap,
    env::{self, Args},
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use serde_json::Value;

pub struct Config {
    local_storage: PathBuf,
}

impl Config {
    fn new(_args: Args) -> Config {
        let path = PathBuf::from("local_storage.json");
        Config {
            local_storage: path,
        }
    }

    pub fn build() -> Result<Config, String> {
        let args = env::args();

        let config = Config::new(args);

        // Creates local json kv store if file does not exist
        if !fs::exists(&config.local_storage).unwrap_or(true) {
            fs::write(&config.local_storage, "").unwrap();
        }

        Ok(config)
    }

    pub fn load_config(&self) -> Result<HashMap<String, String>, String> {
        let mut store: HashMap<String, String> = HashMap::new();

        let file = File::open(&self.local_storage).unwrap();
        let reader = BufReader::new(file);

        let v: Value = serde_json::from_reader(reader).unwrap();

        for (key, value) in v.as_object().unwrap() {
            println!("{:?} {:?}", key, value.as_str().unwrap());
            store.insert(key.to_owned(), value.as_str().unwrap().to_owned());
        }

        Ok(store)
    }
}
