// some kinds of struct

pub fn struct_example() {
    // define width & height
    let w1 = 30;
    let h1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(w1, h1)
    );

    // use tuple
    // this is better than area calculation with two other variables
    let r1 = (35, 59);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect_tuple(r1)
    );

    // use struct
    // this is better than calc with tuple,
    // since struct can give means to its variables
    let r2 = Rectangle {
        width: 45,
        height: 102,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&r2)
    );

    // try to debug
    println!("Rectangle: {:#?}", r2);

    println!("Rectangle method: {}", r2.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rect_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

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

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
