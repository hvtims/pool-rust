pub fn number_logic(num: u32) -> bool {
    let mut oki: Vec<u32> = Vec::new();
    let mut n = num; 
    while n != 0 {
        let digit = n % 10;
        oki.push(digit);
        n /= 10;
    }
    oki.reverse();
    let mut finalll = 0 ;
    for i in oki{
        finalll += i.pow(num.to_string().len().try_into().unwrap());
    }
    if finalll == num{
        return true;
    }
    false
}
