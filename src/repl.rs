use std::{io, path::PathBuf};

use crate::store::memory::{execute_command, parse_arguments};

pub fn runtime(config: crate::config::Config) -> Result<(), String> {
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