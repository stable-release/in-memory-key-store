use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::store::jobs::{Args, Job};

pub fn parse_arguments(
    line: String,
    hashmap: Arc<Mutex<HashMap<String, String>>>,
) -> Result<Args, String> {
    let mut args = line.split_whitespace();

    let command = match args.next() {
        Some("set") => Args {
            command: Job::Set,
            key: args.next().map(|k| k.to_string()),
            value: args.next().map(|v| v.to_string()),
            multiplier: match args.next().map(|m| m.trim().parse::<i64>()) {
                Some(Ok(i)) => Some(i),
                Some(Err(e)) => return Err(format!("{:?}", e)),
                None => Some(1),
            },
            store: hashmap,
        },
        Some("get") => Args {
            command: Job::Get,
            key: args.next().map(|k| k.to_string()),
            value: None,
            multiplier: match args.next().map(|m| m.trim().parse::<i64>()) {
                Some(Ok(i)) => Some(i),
                Some(Err(e)) => return Err(format!("{:?}", e)),
                None => Some(1),
            },
            store: hashmap,
        },
        Some("delete") => Args {
            command: Job::Delete,
            key: match args.next().map(|k| k.to_string()) {
                Some(s) => Some(s),
                None => return Err("Delete must have key".to_string())
            },
            value: None,
            multiplier: match args.next().map(|m| m.trim().parse::<i64>()) {
                Some(Ok(i)) => Some(i),
                Some(Err(e)) => return Err(format!("{:?}", e)),
                None => Some(1),
            },
            store: hashmap,
        },
        Some("exit") => Args {
            command: Job::Exit,
            key: None,
            value: None,
            multiplier: None,
            store: hashmap,
        },
        _ => return Err("Unknown command".to_string()),
    };

    Ok(command)
}
