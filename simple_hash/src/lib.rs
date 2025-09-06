use std::collections::HashMap;
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut newhash = HashMap::new();
    for i in  words {    
        if !newhash.contains_key(i){
            newhash.insert(*i, 1);
        }else{
            let count = newhash.get(i).unwrap() + 1;
            newhash.insert(*i ,count);
        }
    }
    return newhash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    return frequency_count.len();
}
