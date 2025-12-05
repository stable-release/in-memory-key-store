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

    let value: Option<String> = match args.next() {
        Some(arg) => Some(arg.to_string()),
        None => None,
    };

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
    match &command.value {
        Some(value) => {
            if value.len() > 0 {
                Ok(())
            } else {
                Err("Value required length > 0".to_string())
            }
        }
        None => Err("Value required length > 0".to_string()),
    }
}

fn execute_command(
    command: Command,
    store: &mut HashMap<String, String>,
) -> Result<String, String> {
    match command.command {
        Commands::Set => {
            require_key(&command)?;
            require_value(&command)?;
            store.insert(command.key.clone(), command.value.clone().unwrap());
            Ok(format!(
                "key: {}, value: {}",
                command.key,
                command.value.unwrap()
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

pub fn runtime(config: Config) -> Result<(), String> {
    let mut store: HashMap<String, String> = HashMap::new();

    let stdin = io::stdin();
    let lines = stdin.lines();
    for line in lines {
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
