use std::{
    collections::HashMap,
    path::PathBuf,
    process,
    sync::{Arc, Mutex, RwLock, mpsc},
    thread::{self, JoinHandle},
};

use crate::store::persistence::write_local;

#[derive(Debug, PartialEq)]
pub enum Commands {
    Set,
    Get,
    List,
    Delete,
    Clear,
    Exit,
}

impl Clone for Commands {
    fn clone(&self) -> Commands {
        match self {
            Commands::Get => Commands::Get,
            Commands::List => Commands::List,
            Commands::Set => Commands::Set,
            Commands::Delete => Commands::Delete,
            Commands::Clear => Commands::Clear,
            Commands::Exit => Commands::Exit,
        }
    }
}

pub struct Command {
    command: Commands,
    key: String,
    value: Option<String>,
}

impl Clone for Command {
    fn clone(&self) -> Command {
        Command {
            command: self.command.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

pub fn parse_arguments(line: String) -> Result<(Command, i64), String> {
    let mut args = line.split_whitespace();

    let command = match args.next() {
        Some("set") => Commands::Set,
        Some("get") => Commands::Get,
        Some("list") => Commands::List,
        Some("delete") => Commands::Delete,
        Some("clear") => Commands::Clear,
        Some("exit") => Commands::Exit,
        _ => return Err("Unknown command".to_string()),
    };

    let key = args.next().unwrap_or("default").to_string();

    let value = args.next().map(|v| v.to_string());

    let multiplier: i64 = match args.next().map(|v| v.trim().parse::<i64>().unwrap()) {
        Some(n) => n,
        None => 0,
    };

    Ok((
        Command {
            command,
            key,
            value,
        },
        multiplier,
    ))
}

const LOCK: Mutex<bool> = Mutex::new(false);

pub fn execute_command(
    command: Command,
    config_path: PathBuf,
    store: Arc<RwLock<HashMap<String, String>>>,
    multiplier: i64,
) -> Result<Vec<String>, String> {
    // let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // for i in 0..multiplier {
    //     let c = command.clone();
    //     let s = Arc::clone(&store);
    //     let config = config_path.clone();
    //     let handle = thread::spawn(move || {
    //         exec(c, config, s, i).unwrap();
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // Ok("".to_string())
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for i in 0..multiplier {
        let tx = tx.clone();
        let c = command.clone();
        let config = config_path.clone();
        let s = store.clone();
        let increment = i.clone();
        let handle = thread::spawn(move || {
            let result = exec(c, config, s, increment);
            tx.send(result).expect("Channel closed");
        });
        handles.push(handle);
    }

    drop(tx);

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for rec in rx {
        match rec {
            Ok(i) => results.push(i),
            Err(e) => errors.push(e),
        }
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    if !errors.is_empty() {
        eprint!("{} errors occurred:", errors.len());
        for err in &errors {
            eprintln!(" {}", err);
        }

        return Err(errors.into_iter().next().unwrap());
    }

    Ok(results)
}

fn exec(
    command: Command,
    config_path: PathBuf,
    store: Arc<RwLock<HashMap<String, String>>>,
    increment: i64,
) -> Result<String, String> {
    while LOCK.try_lock().is_err() {

    }
    let binding = LOCK;
    let l = binding.lock().unwrap();

    match command.command {
        Commands::Set => {
            require_key(&command)?;
            require_value(&command)?;
            let key = command.key;
            let value = command.value.unwrap();
            while store.try_write().is_err() {

            }
            store.write().unwrap().insert(key.clone(), value.clone());
            write_local(store, config_path).unwrap();
            l.clone();
            Ok::<std::string::String, String>(format!("key: {}, value: {}", key, value))
        }
        Commands::Get => {
            require_key(&command)?;
            match store.read().unwrap().get(&command.key) {
                Some(value) => Ok(format!("key: {}, value: {}", command.key, value)),
                None => Ok(format!("key '{}' does not exist", command.key)),
            }
        }
        Commands::List => {
            let mut list = Vec::new();
            for k in store.read().unwrap().keys() {
                list.push(format!(
                    "key '{}', value '{}'",
                    k,
                    store.read().unwrap().get(k).unwrap()
                ));
            }
            Ok(format!("{:?}", list))
        }
        Commands::Delete => {
            require_key(&command)?;
            store.write().unwrap().remove(&command.key);
            write_local(store, config_path).unwrap();
            Ok(format!("key removed: {}", command.key))
        }
        Commands::Clear => {
            store.write().unwrap().clear();
            write_local(store, config_path).unwrap();
            Ok(format!("database cleared"))
        }
        Commands::Exit => process::exit(0),
    }
}

fn require_key(command: &Command) -> Result<(), String> {
    if command.key.len() > 0 {
        Ok(())
    } else {
        Err("Key required length > 0".to_string())
    }
}

fn require_value(command: &Command) -> Result<(), String> {
    if let Some(value) = &command.value {
        if !value.is_empty() {
            return Ok(());
        }
    }
    Err("Value required length > 0".to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn set_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let key = "&".to_string();
        let value = "b".to_string();
        let (cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let exec = exec(cmd, path, store, multiplier);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json"),
        };
        let v_object = v.as_object().unwrap();
        assert_eq!(v_object.get("&"), Some(&serde_json::to_value("b").unwrap()));
    }

    #[test]
    fn get_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let key = "&".to_string();
        let value = "b".to_string();
        let (mut cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        (cmd, _) = parse_arguments("get &".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Get);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, None);

        e = exec(cmd, path, store, multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json"),
        };
        let v_object = v.as_object().unwrap();
        assert_eq!(v_object.get("&"), Some(&serde_json::to_value("b").unwrap()));
    }

    #[test]
    fn list_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let key = "&".to_string();
        let value = "b".to_string();
        let (mut cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        (cmd, _) = parse_arguments("list".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::List);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        e = exec(cmd, path, store, multiplier);
        assert_eq!(
            e,
            Ok(format!("[\"key '{}', value '{}'\"]", key, "b".to_string()))
        );

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json"),
        };
        let v_object = v.as_object().unwrap();
        assert_eq!(v_object.get("&"), Some(&serde_json::to_value("b").unwrap()));
    }

    #[test]
    fn delete_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let key = "&".to_string();
        let value = "b".to_string();
        let (mut cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        (cmd, _) = parse_arguments("delete &".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Delete);
        assert_eq!(cmd.key, "&");
        assert_eq!(cmd.value, None);

        e = exec(cmd, path, store, multiplier);
        assert_eq!(e, Ok(format!("key removed: &")));

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json"),
        };
        let v_object = v.as_object().unwrap();
        assert_eq!(v_object.get("&"), None);
    }

    #[test]
    fn clear_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let mut key = "&".to_string();
        let mut value = "b".to_string();
        let (mut cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value.clone()));

        let path = PathBuf::from("local_storage.json");
        let mut e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        key = "70".to_string();
        value = "asdf".to_string();
        (cmd, _) = parse_arguments("set 70 asdf".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "asdf".to_string()))
        );

        (cmd, _) = parse_arguments("clear".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Clear);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        e = exec(cmd, path, store, multiplier);
        assert_eq!(e, Ok(format!("database cleared")));

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{}", v.to_string());
    }

    #[test]
    fn exit_no_exit_process() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        let mut key = "&".to_string();
        let mut value = "b".to_string();
        let (mut cmd, multiplier) = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value.clone()));

        let path = PathBuf::from("local_storage.json");
        let mut e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        key = "70".to_string();
        value = "asdf".to_string();
        (cmd, _) = parse_arguments("set 70 asdf".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        e = exec(cmd, path.clone(), store.clone(), multiplier);
        assert_eq!(
            e,
            Ok(format!("key: {}, value: {}", key, "asdf".to_string()))
        );

        (cmd, _) = parse_arguments("clear".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Clear);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        e = exec(cmd, path.clone(), store, multiplier);
        assert_eq!(e, Ok(format!("database cleared")));

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{}", v.to_string());

        (cmd, _) = parse_arguments("exit".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Exit);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);
    }
}
