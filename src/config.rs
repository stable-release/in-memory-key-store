use std::{
    collections::HashMap,
    env::{self, Args},
    fs::{self},
    path::PathBuf, sync::{Arc, Mutex},
};

pub struct Config {
    local_storage: PathBuf,
    pub memory_store: Arc<Mutex<HashMap<String, String>>>
}

impl Config {
    fn new(_args: Args) -> Config {
        let local_storage = PathBuf::from("local_storage.json");
        let memory_store: Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
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
