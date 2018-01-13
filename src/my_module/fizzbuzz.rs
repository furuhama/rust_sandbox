// fizzbuzz function

pub fn fizzbuzz(n: i32) {
    println!("fizz buzz start!");

    for i in 1..n+1 {
        if i%15 == 0 {
            println!("fizzbuzz");
        } else if i%3 == 0 {
            println!("fizz");
        } else if i%5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
    println!("fizz buzz end!");
}

