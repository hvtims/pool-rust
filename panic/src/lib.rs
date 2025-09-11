use std::fs::File;
pub fn open_file(s: &str) -> File {
    let  file = match File::open(s){
        Ok(file) => file,
        Err(msg)=> panic!("{}" ,msg.to_string()),
    };
    return file
}