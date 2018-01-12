use std::io;

pub fn get_and_print_input() {
    println!("please input some words");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line.");
    println!("your input: {}", guess);
}

