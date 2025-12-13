use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}};

use crate::store::persistence::write_local;

// Worker jobs
#[derive(Debug, PartialEq)]
pub enum Job {
    Set,
    Get,
    List,
    Delete,
    Clear,
    Exit,
}

impl Clone for Job {
    fn clone(&self) -> Job {
        match self {
            &Job::Set => Job::Set,
            _ => Job::Exit,
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub command: Job,
    pub key: Option<String>,
    pub value: Option<String>,
    pub multiplier: Option<i64>,
    pub store: Arc<Mutex<HashMap<String, String>>>
}

impl Clone for Args {
    fn clone(&self) -> Args {
        Args {
            command: self.command.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            multiplier: self.multiplier,
            store: Arc::clone(&self.store)
        }
    }
}

impl Args {
    pub fn execute(&self) -> Result<(), String> {
        match self.command {
            Job::Set => set(self.key.as_ref().unwrap(), &self.value.as_ref().unwrap(), self.store.clone()),
            _ => (),
        }

        Ok(())
    }
}

fn set(key: &str, value: &str, store: Arc<Mutex<HashMap<String, String>>>) {
    store.lock().unwrap().insert(key.to_string(), value.to_string());
    let store_clone = Arc::clone(&store);
    let content = serde_json::to_string(&*store_clone.lock().unwrap()).unwrap();

    write_local(content);
}