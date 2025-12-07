use std::{collections::HashMap, fs, io::Write, path::PathBuf};

pub fn write_local(store: &mut HashMap<String, String>, config_path: PathBuf) -> Result<(), String> {
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
