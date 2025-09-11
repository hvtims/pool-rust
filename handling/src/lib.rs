use std::fs::{self, File};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match File::open(path) {
        Ok(_) => {
            fs::write(path, content).unwrap();
        }
        Err(_) => {
            let _ghadir = File::create(path).unwrap();
            fs::write(path, content).unwrap();
        }
    }
}
