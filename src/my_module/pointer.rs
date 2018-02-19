// smart pointers in Rust

use self::List::{Cons, Nil};

pub fn pointers() {
    using_box();
}

fn using_box() {
    // Box is a pointer to heap memory.
    // The pointer itself is located on stack.
    //
    // This memory allocation (on both stack and heap) will deallocate
    // when the function call ends.
    //
    // Box allows recursive types,
    // so Box is sometimes used to implement `cons list`
    // (it is often seen in LISP or its dialect).
    let b = Box::new(5);
    println!("b = {}", b); // b = 5

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    // Box is used as recursive type
    Cons(i32, Box<List>),
    Nil,
}
