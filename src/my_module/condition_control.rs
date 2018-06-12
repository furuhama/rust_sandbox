// basic condition control

pub fn condition_controller() {
    let a = true;
    let b = false;
    println!("boolean: {}, {}", a, b);

    let x = 35;
    // condition control can be used as written in below
    let y = {
        if x % 3 == 0 {
            10
        } else if x % 4 == 0 {
            12
        } else if x % 5 == 0 {
            14
        } else {
            100
        }
    };
    println!("{}", y);

    // while loop
    let mut number = 5;
    while number != 0 {
        println!("{}", number);

        number -= 1;
    }
    println!("while loop end!");

    // for loop
    let array = [10, 20, 30, 40, 50];
    for element in &array {
        println!("array value: {}", element);
    }

    // 1..10 means from 1 to 9 iterator
    for number in 1..10 {
        println!("{}", number);
    }
    for number in (1..10).rev() {
        println!("{}", number);
    }
}
