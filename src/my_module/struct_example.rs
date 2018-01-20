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
    let r1 = (35, 59);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect_tuple(r1)
    );

    // use struct
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

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
