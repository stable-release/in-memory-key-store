use crate::store::jobs::{Args, Job};

pub fn parse_arguments(line: String) -> Result<Args, String> {
    let mut args = line.split_whitespace();

    let command = match args.next() {
        Some("set") => Args {
            command: Job::Set,
            key: args.next().map(|k| k.to_string()),
            value: args.next().map(|v| v.to_string()),
            multiplier: args.next().map(|m| m.trim().parse::<i64>().unwrap()),
        },
        _ => return Err("Unknown command".to_string()),
    };

    Ok(command)
}
