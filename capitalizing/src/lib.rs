pub fn capitalize_first(input: &str) -> String {
    let arr  : Vec<_> = input.chars().collect();
    let mut newstrg = String::new();
    for (i , v ) in arr.into_iter().enumerate(){
        if i == 0 {
            newstrg += &v.to_uppercase().to_string();
            continue;
        }
        newstrg+= &v.to_string();
    }
    return newstrg;
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut word_start = true;
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            word_start = true;
        } else {
            if word_start {
                for up in c.to_uppercase() {
                    result.push(up);
                }
                word_start = false;
            } else {
                for low in c.to_lowercase() {
                    result.push(low);
                }
            }
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    if input.len() == 0 {
        return "".to_string();
    }
    let waw : Vec<_> = input.chars().collect();
    let mut newstr = String::new();
    for i in waw{
        if i.is_lowercase(){
            newstr += &i.to_uppercase().to_string();
            continue;
        }else if i.is_uppercase(){
            newstr += &i.to_lowercase().to_string();
            continue;
        }else{
            newstr+= &i.to_string();
        }
    }
    return newstr
}