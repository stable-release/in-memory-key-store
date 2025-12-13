use std::{
    collections::HashMap,
    process,
    sync::{Arc, Mutex},
};

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
            &Job::Get => Job::Get,
            &Job::List => Job::List,
            &Job::Delete => Job::Delete,
            &Job::Clear => Job::Clear,
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
    pub store: Arc<Mutex<HashMap<String, String>>>,
}

impl Clone for Args {
    fn clone(&self) -> Args {
        Args {
            command: self.command.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            multiplier: self.multiplier,
            store: Arc::clone(&self.store),
        }
    }
}

impl Args {
    pub fn execute(&self) -> Result<(), String> {
        match self.command {
            Job::Set => set(
                self.key.as_ref().unwrap(),
                &self.value.as_ref().unwrap(),
                self.store.clone(),
            ),
            Job::Get => get(self.key.as_ref().unwrap(), self.store.clone()),
            Job::List => list(self.store.clone())?,
            Job::Delete => delete(self.key.as_ref().unwrap(), self.store.clone()),
            Job::Clear => clear(self.store.clone()),
            Job::Exit => exit(),
            _ => (),
        }

        Ok(())
    }
}

fn set(key: &str, value: &str, store: Arc<Mutex<HashMap<String, String>>>) {
    store
        .lock()
        .unwrap()
        .insert(key.to_string(), value.to_string());
    let store_clone = Arc::clone(&store);
    let content = serde_json::to_string(&*store_clone.lock().unwrap()).unwrap();

    write_local(content);
}

fn get(key: &str, store: Arc<Mutex<HashMap<String, String>>>) {
    let binding = store.lock().unwrap();
    let value = binding.get(key);
    match value {
        Some(v) => println!("key: {}, value: {}", key, v),
        None => eprintln!("Key not set"),
    }
}

fn list(store: Arc<Mutex<HashMap<String, String>>>) -> Result<(), String> {
    let binding = store.lock().unwrap();
    let mut pairs = Vec::new();
    for k in binding.keys() {
        pairs.push((
            k.to_string(),
            binding.get(k).unwrap().to_string()
        ));
    }

    println!("{:?}", pairs);

    Ok(())
}

fn delete(key: &str, store: Arc<Mutex<HashMap<String, String>>>) {
    let value = match store.lock().unwrap().remove(key) {
        Some(v) => Some(v),
        None => {
            eprintln!("Missing key: {}", key);
            return;
        }
    };

    let store_clone = Arc::clone(&store);
    let content = serde_json::to_string(&*store_clone.lock().unwrap()).unwrap();

    write_local(content);
    println!(
        "Deleted pair: ({}, {})",
        key,
        value.unwrap_or("".to_string())
    );
}

fn clear(store: Arc<Mutex<HashMap<String, String>>>) {
    store.lock().unwrap().clear();
    let store_clone = Arc::clone(&store);
    let content = serde_json::to_string(&*store_clone.lock().unwrap()).unwrap();
    write_local(content);
    println!("Cleared!");
}

fn exit() {
    process::exit(0);
}
