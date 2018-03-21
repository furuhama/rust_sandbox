// just add one to argument integer

// << use this crate from parent crate like written in below >>
// let num = 10;
// println!("[using adder crate] {} plus one is {}", num, adder::add_one(num));

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
