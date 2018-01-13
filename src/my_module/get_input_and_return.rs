// get input as guess and return answer

extern crate rand;

use std::io;
use self::rand::Rng; // module内で外部モジュールを使う時は self::hoge とする

pub fn get_and_print_input() {
    println!("please input some words");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess) // io::stdin().read_line(hoge) は io::Result インスタンス
        .expect("failed to read line.");
        // expect は io::Result が err のときにその命令を停止して、引数の文字列を標準出力から返す
    println!("your input: {}", guess);
}

pub fn guess_the_number() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("you guessed: {}", guess);
}

