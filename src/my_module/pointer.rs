// smart pointers in Rust

use std::ops::Deref;
use std::rc::Rc;

pub fn pointers() {
    using_box();
    drop_trait();
    rc_trait();
    ref_cell();
}

fn using_box() {
    use self::List::{Cons, Nil};

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

// Rc is an abbreviation for `reference counting`.
fn rc_trait() {
    use self::RList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}

fn ref_cell() {
}

#[derive(Debug)]
enum List {
    // Box is used as recursive type
    Cons(i32, Box<List>),
    Nil,
}

enum RList {
    Cons(i32, Rc<RList>),
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

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}
