pub fn factorial(num: u64) -> u64 {
    let mut ok = 1 ;
    for i in 2..=num{
        ok *= i;
    }
    return ok;
}