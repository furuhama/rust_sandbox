// match

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // Coin::Penny => 1,
        // we can write some process as a result of matching
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn match_control() {
    let c = Coin::Penny;
    println!("{}", value_in_cents(c));

    let q = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(q));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}", six, none);

    let some_u8_value = 0u8;
    let mut count = 0;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => count += 1,
    }
    println!("count: {}", count);

    // let coin = Coin::Quarter(UsState::Alaska);
    let coin = Coin::Penny;

    if let Coin::Quarter(state) = coin {
        println!("hoge hoge {:?}", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}
