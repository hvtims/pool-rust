use std::io; 
fn main() {
    let mut count = 0 ;
    let answer = "The letter e";
    loop  {
         println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
    .expect("failed to read");
        count+= 1 ;
        if input.trim() == answer{
          
            break;
        }
    }
    
    println!("Number of trials: {}", count);
}
