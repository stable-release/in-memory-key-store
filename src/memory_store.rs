use std::{
    collections::HashMap, fmt::format, fs::{self, OpenOptions}, io::{self, Write}, path::PathBuf, process
};

use crate::config::Config;

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
    fn execute(
        &self,
        store: &mut HashMap<String, String>,
        config_path: PathBuf,
    ) -> Result<(), String> {
        let overwrite_path = if !fs::exists(&config_path).unwrap_or(true) {
            let p = PathBuf::from("local_storage_overwite.json");
            fs::write(&p, "").unwrap();
            p
        } else {
            let p = config_path.to_str().unwrap();
            let p = PathBuf::from(p.replace(".json", "_overwrite.json"));
            fs::write(&p, "").unwrap();
            p
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&overwrite_path)
            .unwrap();

        // First write to new file
        let convert = serde_json::to_string(store).unwrap();
        file.write_all(convert.as_bytes()).unwrap();
        // Then remove old file
        fs::remove_file(&config_path).unwrap();
        // Rename overwrite as main local storage file
        fs::rename(overwrite_path, config_path).unwrap();
        Ok(())
    }
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

fn execute_command(
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
            command.command.execute(store, config_path).unwrap();

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
            command.command.execute(store, config_path).unwrap();
            Ok(format!("key removed: {}", command.key))
        }
        Commands::Clear => {
            store.clear();
            command.command.execute(store, config_path).unwrap();
            Ok(format!("database cleared"))
        }
        Commands::Exit => process::exit(0),
    }
}

// Loading Config for possible persistence
pub fn runtime(config: Config) -> Result<(), String> {
    let mut store = match config.load_config() {
        Ok(s) => s,
        Err(e) => return Err(e),
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

        let path = match config.return_local_storage_path() {
            Ok(p) => p,
            Err(_) => PathBuf::from("local_storage_overwrite.json"),
        };

        match execute_command(command, path, &mut store) {
            Ok(output) => println!("{}", output),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }

    Ok(())
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
