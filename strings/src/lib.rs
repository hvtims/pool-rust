pub fn char_length(s: &str) -> usize {
    let arr : Vec<_>= s.chars().collect();
    return arr.len()
}