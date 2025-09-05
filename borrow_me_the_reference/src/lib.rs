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
    for eq in v.iter_mut() {
        let parts: Vec<&str> = eq.split_whitespace().collect();
        
        if parts.len() == 3 {
            let left_operand: i32 = parts[0].parse().unwrap();
            let operator = parts[1];
            let right_operand: i32 = parts[2].parse().unwrap();
            
            let result = match operator {
                "+" => left_operand + right_operand,
                "-" => left_operand - right_operand,
                _ => {
                    continue;
                }
            };
                        *eq = result.to_string();
        }
    }
}
