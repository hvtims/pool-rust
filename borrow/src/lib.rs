pub fn str_len(s:&str ) -> usize {
    let sstringg = s.to_string();
    let mut vectt = vec![];
    for i in sstringg.chars(){
        vectt.push(i);
    }
    return vectt.len()
}