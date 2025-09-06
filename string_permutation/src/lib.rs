pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len(){
        return false;
    }
    let neededlen = s1.len();
    let mut count = 0;
    for i in s1.chars() {
        for k in s2.chars(){
            if k == i {
                count += 1 ;
            }
        }
    }
    if count < neededlen{
        return false
    }
    return true
}