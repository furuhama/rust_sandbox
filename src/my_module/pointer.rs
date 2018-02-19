// smart pointers in Rust

pub fn pointers() {
    using_box();
}

fn using_box() {
    let b = Box::new(5);
    println!("b = {}", b); // b = 5
}
