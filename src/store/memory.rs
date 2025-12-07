use std::{collections::HashMap, path::PathBuf, process};

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

impl Commands {
    // fn execute(&self, key: Option<&str>, value: Option<&str>) -> Result<(), String> {
    //     match self {
    //         Commands::Set => Ok(()),
    //         Commands::Delete => Ok(()),
    //         Commands::Clear => Ok(()),
    //         _ => Ok(())
    //     }
    // }
}

pub struct Command {
    command: Commands,
    key: String,
    value: Option<String>,
}

pub fn parse_arguments(line: String) -> Result<Command, String> {
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

    Ok(Command {
        command,
        key,
        value,
    })
}

pub fn execute_command(
    command: Command,
    config_path: PathBuf,
    store: &mut HashMap<String, String>,
) -> Result<String, String> {
    match command.command {
        Commands::Set => {
            require_key(&command)?;
            require_value(&command)?;
            let key = command.key;
            let value = command.value.unwrap();
            store.insert(key.clone(), value.clone());
            write_local(store, config_path).unwrap();

            Ok(format!("key: {}, value: {}", key, value))
        }
        Commands::Get => {
            require_key(&command)?;
            match store.get(&command.key) {
                Some(value) => Ok(format!("key: {}, value: {}", command.key, value)),
                None => Ok(format!("key '{}' does not exist", command.key)),
            }
        }
        Commands::List => {
            let mut list = Vec::new();
            for k in store.keys() {
                list.push(format!("key '{}', value '{}'", k, store.get(k).unwrap()));
            }
            Ok(format!("{:?}", list))
        }
        Commands::Delete => {
            require_key(&command)?;
            store.remove(&command.key);
            write_local(store, config_path).unwrap();
            Ok(format!("key removed: {}", command.key))
        }
        Commands::Clear => {
            store.clear();
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
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let key = "&".to_string();
        let value = "b".to_string();
        let cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let exec = execute_command(cmd, path, &mut store);
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
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{\"&\":\"b\"}", v.to_string());
    }

    #[test]
    fn get_valid() {
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let key = "&".to_string();
        let value = "b".to_string();
        let mut cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        cmd = parse_arguments("get &".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Get);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, None);

        exec = execute_command(cmd, path, &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{\"&\":\"b\"}", v.to_string());
    }

    #[test]
    fn list_valid() {
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let key = "&".to_string();
        let value = "b".to_string();
        let mut cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        cmd = parse_arguments("list".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::List);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        exec = execute_command(cmd, path, &mut store);
        assert_eq!(
            exec,
            Ok(format!("[\"key '{}', value '{}'\"]", key, "b".to_string()))
        );

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{\"&\":\"b\"}", v.to_string());
    }

    #[test]
    fn delete_valid() {
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let key = "&".to_string();
        let value = "b".to_string();
        let mut cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        let path = PathBuf::from("local_storage.json");
        let mut exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        cmd = parse_arguments("delete &".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Delete);
        assert_eq!(cmd.key, "&");
        assert_eq!(cmd.value, None);

        exec = execute_command(cmd, path, &mut store);
        assert_eq!(exec, Ok(format!("key removed: &")));

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
    fn clear_valid() {
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let mut key = "&".to_string();
        let mut value = "b".to_string();
        let mut cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value.clone()));

        let path = PathBuf::from("local_storage.json");
        let mut exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        key = "70".to_string();
        value = "asdf".to_string();
        cmd = parse_arguments("set 70 asdf".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "asdf".to_string()))
        );

        cmd = parse_arguments("clear".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Clear);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        exec = execute_command(cmd, path, &mut store);
        assert_eq!(exec, Ok(format!("database cleared")));

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
        let _config = crate::config::Config::build().unwrap();
        let mut store = HashMap::new();
        let mut key = "&".to_string();
        let mut value = "b".to_string();
        let mut cmd = parse_arguments("set & b".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value.clone()));

        let path = PathBuf::from("local_storage.json");
        let mut exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "b".to_string()))
        );
        assert!(fs::exists("local_storage.json").unwrap());

        key = "70".to_string();
        value = "asdf".to_string();
        cmd = parse_arguments("set 70 asdf".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Set);
        assert_eq!(cmd.key, key);
        assert_eq!(cmd.value, Some(value));

        exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(
            exec,
            Ok(format!("key: {}, value: {}", key, "asdf".to_string()))
        );

        cmd = parse_arguments("clear".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Clear);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);

        exec = execute_command(cmd, path.clone(), &mut store);
        assert_eq!(exec, Ok(format!("database cleared")));

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{}", v.to_string());

        cmd = parse_arguments("exit".to_string()).unwrap();
        assert_eq!(cmd.command, Commands::Exit);
        assert_eq!(cmd.key, "default");
        assert_eq!(cmd.value, None);
    }
}
