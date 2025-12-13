use std::{sync::mpsc, thread};

use crate::store::jobs::{Args};

pub fn start_worker() -> mpsc::Sender<Args> {
    let (tx, rx) = mpsc::channel::<Args>();

    thread::spawn(move || {
        while let Ok(args) = rx.recv() {
            args.execute().unwrap();
        }
    });

    tx
}