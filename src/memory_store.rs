use std::{collections::HashMap, io, process};

use crate::config::Config;

enum Commands {
    Set,
    Get,
    Delete,
    Clear,
    Exit,
}

struct Command {
    command: Commands,
    key: String,
    value: Option<String>,
}

fn parse_arguments(line: String) -> Result<Command, String> {
    let mut args = line.split_whitespace();

    let command = match args.next() {
        Some("set") => Commands::Set,
        Some("get") => Commands::Get,
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
            return Ok(())
        }
    }
    Err("Value required length > 0".to_string())
}

fn execute_command(
    command: Command,
    store: &mut HashMap<String, String>,
) -> Result<String, String> {
    match command.command {
        Commands::Set => {
            require_key(&command)?;
            require_value(&command)?;
            let key = command.key;
            let value = command.value.unwrap();
            store.insert(key.clone(), value.clone());
            Ok(format!(
                "key: {}, value: {}",
                key,
                value
            ))
        }
        Commands::Get => {
            require_key(&command)?;
            match store.get(&command.key) {
                Some(value) => Ok(format!("key: {}, value: {}", command.key, value)),
                None => Ok(format!("key '{}' does not exist", command.key)),
            }
        }
        Commands::Delete => {
            require_key(&command)?;
            store.remove(&command.key);
            Ok(format!("key removed: {}", command.key))
        }
        Commands::Clear => {
            store.clear();
            Ok(format!("database cleared"))
        }
        Commands::Exit => process::exit(0),
    }
}

// Loading Config for possible persistence
pub fn runtime(config: Config) -> Result<(), String> {
    let mut store = match config.load_config() {
        Ok(s) => s,
        Err(e) => {
            return Err(e)
        }
    };

    // let mut store: HashMap<String, String> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lines() {
        let command = match parse_arguments(line.unwrap()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match execute_command(command, &mut store) {
            Ok(output) => println!("{}", output),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }

    Ok(())
}
