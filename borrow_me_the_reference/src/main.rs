use borrow_me_the_reference::*;

fn main() {
    let mut a = "borr+++aaaow".to_owned();
    // let mut b = [
    //     "2+2".to_owned(),
    //     "3+2".to_owned(),
    //     "10-3".to_owned(),
    //     "5+5".to_owned(),
    // ];

    delete_and_backspace(&mut a);
    // do_operations(&mut b);

    println!("{:?}", (a));
}