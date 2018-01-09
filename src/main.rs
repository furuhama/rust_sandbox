use std::io;

fn main() {
    println!("hello, rust!");

    println!("please input some words");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("Input: {}", guess);
}
