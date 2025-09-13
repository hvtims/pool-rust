pub fn score(a: &str) -> u64 {
    let lowercase = a.to_lowercase();
    let mut total = 0;

    for c in lowercase.chars() {
        total += match c {
            'a' | 'e' | 'i' | 'o' | 'u' |
            'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        };
    }

    return total;
}
