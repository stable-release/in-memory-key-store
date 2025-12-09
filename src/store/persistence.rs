use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub fn write_local(
    store: Arc<RwLock<HashMap<String, String>>>,
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

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&overwrite_path)
        .unwrap();

    // First write to new file
    let store_clone = store.read().unwrap();
    let convert = serde_json::to_string(&*store_clone).unwrap();
    file.write_all(convert.as_bytes()).unwrap();
    // Then remove old file
    fs::remove_file(&config_path).unwrap();
    // Rename overwrite as main local storage file
    fs::rename(overwrite_path, config_path).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::store::persistence::write_local;

    #[test]
    fn write_local_valid() {
        let config = crate::config::Config::build().unwrap();
        let store = config.load_config().unwrap();
        store
            .write()
            .unwrap()
            .insert("vKey".to_string(), "kValue".to_string());
        let write = write_local(store, config.return_local_storage_path().unwrap());
        assert_eq!(write, Ok(()));
        assert!(std::fs::exists("local_storage.json").unwrap());
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) if e.is_eof() => serde_json::json!({}),
            Err(_e) => panic!("Invalid json"),
        };
        let v_object = v.as_object().unwrap();
        assert_eq!(v_object.get("vKey"), Some(&serde_json::to_value("kValue").unwrap()));
    }
}
