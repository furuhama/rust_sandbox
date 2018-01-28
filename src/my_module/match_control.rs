// match

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

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
}
