// fizzbuzz function

pub fn fizzbuzz(n: i32) {
    println!("fizz buzz start!");

    for_fizzbuzz(n);

    match_fizzbuzz(n);

    println!("fizz buzz end!");
}

fn for_fizzbuzz(n: i32) {
    for i in 1..n+1 {
        if i%15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn match_fizzbuzz(n: i32) {
    for i in 1..n+1 {
        match i%15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
