use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut hashihashi = HashMap::new();
    let mut count = 0;

    for i in s1.chars() {
        for k in s2.chars() {
            if i == k {
                if hashihashi.contains_key(&i) {
                    continue;
                } else {
                    hashihashi.insert(i, true);
                    count += 1;
                }
            }
        }
    }

    count == s2.len()
}
