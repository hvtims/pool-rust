pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c >= 'a' && c <= 'z' {
            let mut shifted = (c as u8 - b'a') as i8 + key;
            if shifted < 0 {
                shifted += 26;
            } else if shifted >= 26 {
                shifted -= 26;
            }
            result.push((b'a' + shifted as u8) as char);
        } else if c >= 'A' && c <= 'Z' {
            let mut shifted = (c as u8 - b'A') as i8 + key;
            if shifted < 0 {
                shifted += 26;
            } else if shifted >= 26 {
                shifted -= 26;
            }
            result.push((b'A' + shifted as u8) as char);
        } else {
            result.push(c);
        }
    }

    result
}
