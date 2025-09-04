pub fn rev_str(input: &str) -> String {
    let mut salina = String::new();
    for i in input.chars().rev() {
        salina.push(i);
    }

    return salina;
}