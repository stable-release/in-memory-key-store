use std::{collections::HashMap, fs, path::PathBuf, sync::{Arc, Mutex}};

pub fn write_local(contents: String) {
    let path = PathBuf::from("local_storage_overwite.json");

    fs::write(path, contents).unwrap();
}