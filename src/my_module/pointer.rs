// smart pointers in Rust

use self::List::{Cons, Nil};
use std::ops::Deref;

pub fn pointers() {
    using_box();
    drop_trait();
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

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
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
    assert_eq!(5, *(q.deref())); // after defined Deref trait, these two values(5 & q) can be compared.

    let r = MyBox::new(String::from("Rust"));
    hello(&r);

    let s = MyBox2::new(String::from("Haskell"));
    hello(&(*s)[..]);
}

fn drop_trait() {
    {
        // Use Drop trait
        let _c = CustomSmartPointer { data: String::from("hogeee") };
        let _d = CustomSmartPointer { data: String::from("fugaaaaa") };
        println!("CustomSmartPointers created.");
        // at this line, Drop trait's drop() called automatically.
    }
    let e = CustomSmartPointer { data: String::from("piyoooo") };
    println!("CustomSmartPointer e is created.");
    // e.drop(); <- this method calling is not allowed in Rust
    drop(e);
    println!("CustomSmartPointer e dropped before the end of main function.");
}

#[derive(Debug)]
enum List {
    // Box is used as recursive type
    Cons(i32, Box<List>),
    Nil,
}

// define MyBox behaves life Box type
struct MyBox<T>(T);

// define MyBox type behavior in this section
impl<T> MyBox<T> {
    // new() gets any types(T) of value, and it returns MyBox<T> type value
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// to dereference a value (= `*hoge`),
// Deref trait is necessary.
// In other words, to define * operator, Deref trait is needed.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct MyBox2<T>(T);

impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2(x)
    }
}

impl<T> Deref for MyBox2<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
