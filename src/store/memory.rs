use std::{
    collections::HashMap, path::PathBuf, process
};

use crate::store::persistence::{write_local};

enum Commands {
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
