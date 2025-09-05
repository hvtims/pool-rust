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

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        let mut rrrr = 0;
        let mut ssss = ' ';

        for (i, c) in s.chars().enumerate() {
            if c == '+' || c == '-' {
                rrrr = i;
                ssss = c;
                break;
            }
        }

        let left: i32 = s[..rrrr].parse().unwrap();
        let right: i32 = s[rrrr + 1..].parse().unwrap();

        let res = if ssss == '+' {
            left + right
        } else {
            left - right
        };

        *s = res.to_string();
    }
}
