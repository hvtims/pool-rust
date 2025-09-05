pub fn first_subword(mut s: String) -> String {
    let mut ghadir = String::new();
    for (i , v) in s.chars().enumerate() {
        if (v.to_string() == "_".to_string()  || v.is_uppercase()) && i != 0 {
            break;
        }
        ghadir += &v.to_string();
    }
   
    return ghadir
}