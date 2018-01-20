// Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl definitions can be separated
impl Rectangle {
    // this function is called method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this function is called associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn method_syntax() {
    let r1 = Rectangle {
        width: 45,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", r1.area());

    let r2 = Rectangle {
        width: 40,
        height: 40,
    };
    let r3 = Rectangle {
        width: 30,
        height: 100,
    };
    println!("Can r1 hold r2?: {}", r1.can_hold(&r2));
    println!("Can r1 hold r3?: {}", r1.can_hold(&r3));

    let sq1 = Rectangle::square(10);
    println!("Square: {:#?}", sq1);
}
