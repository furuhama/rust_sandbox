// smart pointers in Rust

use self::List::{Cons, Nil};
use std::ops::Deref;

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

    // Use Box like referencing
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Use MyBox like Box
    let p = 5;
    let q = MyBox::new(p);

    assert_eq!(5, p);
    assert_eq!(5, *(q.deref()));

    let r = MyBox::new(String::from("Rust"));
    hello(&r);
}

#[derive(Debug)]
enum List {
    // Box is used as recursive type
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
