use arrays::*;

fn main() {
    let a : Vec<_> = (1..=10).collect();
    let b : Vec<_>= [5;10].into_iter().collect();

    println!("The sum of the elements in  is {}", sum(a));
    println!("The sum of the elements in  is {}", sum(b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}