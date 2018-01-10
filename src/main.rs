// use std::io;

fn main() {
    println!("==== hello, rust! ====");

    // println!("please input some words");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("failed to read line");
    // println!("Input: {}", guess);

    let x: i32 = 2836;
    let y: i32 = 10;
    println!("x: {}, y: {}", x, y);

    let mut count: i32 = 0;
    loop {
        count += 1;
        if count % 15 == 0 {
            println!("fizzbuzz");
            continue;
        } else if count % 3 == 0 {
            println!("fizz");
            continue;
        } else if count % 5 == 0 {
            println!("buzz");
            continue;
        } else {
            println!("{}", count);
        }

        if count == 29 {
            break;
        }
    }
}
