pub fn is_pangram(s: &str) -> bool {
    let mut seen = [false; 26];
    for ch in s.bytes() {
        if ch.is_ascii_alphabetic() {
            let idx = (ch.to_ascii_lowercase() - b'a') as usize;
            seen[idx] = true;
        }
    }
    for b in seen.iter() {
    if !b {
        return false;
    }
}
true

}
