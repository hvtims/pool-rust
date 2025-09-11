pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
