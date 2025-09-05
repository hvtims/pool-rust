pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by(|a, b| {
        let a_digit = a.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');
        let b_digit = b.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');
        a_digit.cmp(&b_digit)
    });

    let mut result = String::new();
    for word in words {
        for c in word.chars() {
            if !c.is_ascii_digit() {
                result.push(c);
            }
        }
        result.push(' ');
    }

    result.trim_end().to_string()
}
