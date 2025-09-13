pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    let target = (c as u8 - b'A') as usize;

    let build_line = |i: usize| {
        let letter = (b'A' + i as u8) as char;
        let leading_spaces = target - i;
        if i == 0 {
            " ".repeat(leading_spaces) + &letter.to_string() + &" ".repeat(leading_spaces)
        } else {
            let inner_spaces = 2 * i - 1;
            " ".repeat(leading_spaces) + &letter.to_string() + &" ".repeat(inner_spaces) + &letter.to_string() + &" ".repeat(leading_spaces)
        }
    };

    for i in 0..=target {
        result.push(build_line(i));
    }
    for i in (0..target).rev() {
        result.push(build_line(i));
    }

    result
}
