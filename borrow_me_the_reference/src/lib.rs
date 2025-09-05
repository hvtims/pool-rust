pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut count = 0 ;
    for c in s.chars() {
        if c == '-' {
            if !result.is_empty() {
                result.pop();
            }
        } else if c == '+' {
            count+= 1;
            continue;
        } else {
            if count == 0 {
                result.push(c);
            }else{
                count -= 1;
            }
        }
    }
    *s = result; 
}


// pub fn do_operations(v: &mut [String]) {
// }