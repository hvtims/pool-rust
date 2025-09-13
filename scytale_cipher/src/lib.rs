pub fn scytale_cipher(message: &str, rows: usize) -> String {
    if message.is_empty() || rows == 0 {
        return String::new();
    }

    let len = message.len();
    let cols = (len + rows - 1) / rows;
    let mut result = String::new();

    for row in 0..rows {
        for col in 0..cols {
            let idx = col * rows + row;
            if idx < len {
                result.push(message.chars().nth(idx).unwrap());
            } else {
                result.push(' ');
            }
        }
    }

    result.trim_end().to_string()
}
