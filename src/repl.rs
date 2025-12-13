use std::{io, thread};

use crate::{config::Config, store::workers::start_worker};

// NEW REPL
// Spawns worker threads for execution and command handling

///
/// Main Runtime
/// 
pub fn runtime(config: Config) -> Result<(), i64> {
    let tx = start_worker();

    let stdin = io::stdin();
    for line in stdin.lines() {
        let mut handles = vec![];
        let mut args = line.unwrap().split_whitespace();
    }


    Ok(())
}