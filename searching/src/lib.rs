pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut result = None;

    for (i, &v) in array.iter().enumerate() {
        if v == key {
            result = Some(i);
        }
    }

    result 
}
