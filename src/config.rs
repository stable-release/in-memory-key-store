use std::{
    collections::HashMap, env::{self, Args}, fs::{self}, io::BufReader, path::PathBuf, sync::{Arc, Mutex}
};

use serde_json::Value;

pub struct Config {
    local_storage: PathBuf,
    pub memory_store: Arc<Mutex<HashMap<String, String>>>
}

impl Config {
    fn new(_args: Args) -> Config {
        let local_storage = PathBuf::from("local_storage.json");

        let mut hashmap: HashMap<String, String> = HashMap::new();
        if fs::exists(&local_storage).unwrap() {
            let file = fs::OpenOptions::new().read(true).open(&local_storage).unwrap();
            let reader = BufReader::new(file);
            let v: Value = serde_json::from_reader(reader).unwrap();

            for (key, value) in v.as_object().unwrap() {
                hashmap.insert(key.to_string(), value.to_string());
            }
        }

        let memory_store: Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(hashmap));

        Config {
            local_storage,
            memory_store
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

    pub fn return_local_storage_path(&self) -> Result<PathBuf, String> {
        Ok(self.local_storage.clone())
    }
    
}
