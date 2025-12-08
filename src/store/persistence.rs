use std::{collections::HashMap, fs, io::Write, path::PathBuf, sync::{Arc, RwLock}};

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
    let convert = serde_json::to_string(store).unwrap();
    file.write_all(convert.as_bytes()).unwrap();
    // Then remove old file
    fs::remove_file(&config_path).unwrap();
    // Rename overwrite as main local storage file
    fs::rename(overwrite_path, config_path).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::store::persistence::write_local;

    #[test]
    fn write_local_valid() {
        let config = crate::config::Config::build().unwrap();
        let mut store: HashMap<String, String> = std::collections::HashMap::new();
        store.insert("vKey".to_string(), "kValue".to_string());
        let write = write_local(&mut store, config.return_local_storage_path().unwrap());
        assert_eq!(write, Ok(()));
        assert!(std::fs::exists("local_storage.json").unwrap());
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("local_storage.json")
            .unwrap();
        let reader = std::io::BufReader::new(file);
        let v: serde_json::Value = serde_json::from_reader(reader).unwrap();
        assert_eq!("{\"vKey\":\"kValue\"}", v.to_string());
    }
}
