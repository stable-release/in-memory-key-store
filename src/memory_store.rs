use std::{collections::HashMap, io, process};

use crate::config::Config;

enum Commands {
    Set,
    Get,
    Delete,
}

struct Command {
    command: Commands,
    key: String,
    value: Option<String>,
}

fn parse_arguments(line: String) -> Result<Command, String> {
    let l = line.clone();

    let mut args = l.split_whitespace();

    let command = match args.next().unwrap() {
        "set" => Commands::Set,
        "get" => Commands::Get,
        "delete" => Commands::Delete,
        "exit" => process::exit(0),
        _ => return Err("Unknown command".to_string()),
    };

    let key = match args.next() {
        Some(arg) => arg.to_string(),
        None => return Err("No name found".to_string()),
    };

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

fn execute_command(command: Command, store: &mut HashMap<String, String>) -> Result<(), String> {
    match command.command {
        Commands::Set => {
            store.insert(command.key.clone(), command.value.clone().unwrap());
            println!("key: {}, value: {}", command.key, command.value.unwrap());
            Ok(())
        }
        Commands::Get => {
            match store.get(&command.key) {
                Some(value) => println!("key: {}, value: {}", command.key, value),
                None => println!("key '{}' does not exist", command.key),
            }
            Ok(())
        }
        Commands::Delete => {
            store.remove(&command.key);
            println!("key removed: {}", command.key);
            Ok(())
        }
    }
}

pub fn runtime(config: Config) -> Result<(), String> {
    let mut store: HashMap<String, String> = HashMap::new();

    loop {
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

            execute_command(command, &mut store).unwrap()
        }
    }
}
