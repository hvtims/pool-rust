use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut hashihashi: HashMap<char, usize> = HashMap::new();
    let mut count = 0;

    for i in s1.chars() {
        for k in s2.chars() {
            if i == k {
                let entry = hashihashi.entry(k).or_insert(0);
                if *entry > 0 {
                    continue;
                } else {
                    *entry += 1; 
                    count += 1;
                    break;
                }
            }
        }
    }

    count == s1.chars().count()
}
