pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") || message.len() == 0{
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
