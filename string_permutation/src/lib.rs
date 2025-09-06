use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut count1 = HashMap::new();
    let mut count2 = HashMap::new();

    for c in s1.chars() {
        *count1.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *count2.entry(c).or_insert(0) += 1;
    }

    count1 == count2
}
