pub fn sum(a: Vec<i32>) -> i32 {
    let mut counter = 0 ; 
    for i in a {
        counter += i;
    }
    return counter ;
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let ok = [10;32];
    return ok
}