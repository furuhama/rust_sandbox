// Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn method_syntax() {
    let r1 = Rectangle {
        width: 45,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", r1.area());
}
