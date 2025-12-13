use std::{sync::mpsc, thread};

use crate::store::jobs::Job;

pub fn start_worker() -> mpsc::Sender<Job> {
    let (tx, rx) = mpsc::channel::<Job>();

    thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            match job {
                Job::Get(args) => (),
                Job::Set(args) => (),
                Job::Clear(args) => (),
                _ => ()
            }
            break;
        }
    });

    tx
}