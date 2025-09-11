use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match File::open(path) {
        Ok(_) => {
            let mut ghadir = OpenOptions::new()
                .append(true)
                .open(path)
                .unwrap();
            let _ = ghadir.write_all(content.as_bytes());
        }
        Err(_) => {
            let mut ghadir = File::create(path).unwrap();
            let _ = ghadir.write_all(content.as_bytes());
        }
    }
}
