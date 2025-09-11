pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(), 
        Security::Message => server.expect("ERROR: program stops"),
        Security::Warning => match server {
            Ok(url) => url,
            Err(_) => "WARNING: check the server".to_string(),
        },
        Security::NotFound => match server {
            Ok(url) => url,
            Err(msg) => format!("Not found: {}", msg),
        },
        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(msg) => msg,
        },
    }
}
