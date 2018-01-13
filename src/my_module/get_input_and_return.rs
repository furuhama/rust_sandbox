// get input as guess and return answer

extern crate rand;

use std::io;
use std::cmp::Ordering;
use self::rand::Rng;
// module内で外部モジュールを extern して使う時は self::hoge とする

pub fn guess_the_number() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // io::stdin().read_line(hoge) は io::Result インスタンス
            .expect("failed to read line");
            // expect は io::Result が err のときにその命令を停止して、引数の文字列を標準出力から返す

        let guess: u32 = guess.trim().parse()
            .expect("please input positive integer!!!");

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => println!("you win!"),
        }
    }
}

