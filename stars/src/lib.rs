pub fn stars(n: u32) -> String {
    let powpow = 2_i32.pow(n);
    return  "*".repeat(powpow.try_into().unwrap()).to_string();
}