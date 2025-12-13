use std::{
    fs,
    io::Write,
    path::PathBuf,
};

pub fn write_local(contents: String) {
    let overwrite_path = PathBuf::from("local_storage_overwite.json");
    let path = PathBuf::from("local_storage.json");

    if !fs::exists(&overwrite_path).unwrap() {
        fs::write(&overwrite_path, "").unwrap();
    }

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&overwrite_path)
        .unwrap();

    file.write_all(contents.as_bytes()).unwrap();
    fs::remove_file(&path).unwrap();
    fs::rename(overwrite_path, path).unwrap();
}
