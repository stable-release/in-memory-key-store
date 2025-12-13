use std::{io, sync::Arc, thread};

use crate::{
    config::Config,
    store::{
        builder::parse_arguments,
        workers::start_worker,
    },
};

// NEW REPL
// Spawns worker threads for execution and command handling

///
/// Main Runtime
///
pub fn runtime(config: Config) -> Result<(), String> {
    let worker_tx = start_worker();

    let stdin = io::stdin();
    for line in stdin.lines() {
        let mut handles = vec![];
        let job = parse_arguments(line.unwrap(), Arc::clone(&config.memory_store))?;

        let multiplier = &job.multiplier.unwrap();

        for _ in 0..*multiplier {
            let tx = worker_tx.clone();
            let j = job.clone();

            handles.push(thread::spawn(move || {
                match tx.send(j) {
                    Ok(()) => (),
                    Err(e) => eprintln!("{}", e),
                };
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // worker_tx.send(Args {
        //     command: Job::Exit,
        //     key: None,
        //     value: None,
        //     multiplier: None,
        // });
    }

    Ok(())
}
