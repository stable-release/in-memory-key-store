use std::{
    collections::HashMap,
    env::{self, Args},
    fs::{self, File},
    io::BufReader,
    path::PathBuf, sync::{Arc, RwLock},
};

use serde_json::Value;

pub struct Config {
    local_storage: PathBuf,
    memory_store: Arc<RwLock<HashMap<String, String>>>
}

impl Config {
    fn new(_args: Args) -> Config {
        let local_storage = PathBuf::from("local_storage.json");
        let memory_store: Arc<RwLock<HashMap<String,String>>> = Arc::new(RwLock::new(HashMap::new()));
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

    pub fn load_config(&self) -> Result<Arc<RwLock<HashMap<String, String>>>, String> {
        let store = Arc::clone(&self.memory_store);
        let file = File::open(&self.local_storage).unwrap();
        let reader = BufReader::new(file);

        let v: Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json")
        };

        for (key, value) in v.as_object().unwrap() {
            // println!("{:?} {:?}", key, value.as_str().unwrap());
            store.write().unwrap().insert(key.to_owned(), value.as_str().unwrap().to_owned());
        }

        Ok(store)
    }

    pub fn return_local_storage_path(&self) -> Result<PathBuf, String> {
        Ok(self.local_storage.clone())
    }
    
}
