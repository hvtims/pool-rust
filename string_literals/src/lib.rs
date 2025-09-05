pub fn is_empty(v: &str) -> bool {
    if v.len() == 0{
        return true ;
    }
    return false ;
}

pub fn is_ascii(v: &str) -> bool {
    for i in v.chars(){
        let u = i as u8 ;
        if u< 32 || u > 126{
            return false ;
        }
    }
    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    if v.contains(pat){
        return true ;
    }
    return false ;
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
        return v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
     return v.find(pat).unwrap()
}