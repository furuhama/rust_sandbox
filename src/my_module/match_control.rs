// match

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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
        Coin::Quarter => 25,
    }
}

pub fn match_control() {
    let c = Coin::Penny;

    println!("{}", value_in_cents(c));
}
