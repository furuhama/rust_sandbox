// References and Borrowings

pub fn references_and_borrowings() {
    // define x as mutable (default: i32) value,
    // to edit x later in child scope
    let mut x = 5;
    // make new child scope
    {
        // make &mut reference only in child scope
        let y = &mut x;
        // to use referenced value(= `x`),
        // add * mark at the head of `y`
        *y += 1;
        // at the end of this scope,
        // the reference of `x` by `y` ends
    }
    // `println!` macro references argument as an immutable variable
    // (= & reference, not &mut reference)
    println!("use &mut reference: {}", x);

    let num = &5;
    let fo = Foo { x: num };
    println!("fo's x is: {}", fo.x_value());
}

// struct Foo has lifetime `a`
struct Foo<'a> {
    // its field x is a reference, so it also has lifetime `a`
    x: &'a i32,
}

// function x_value has lifetime 'a
impl<'a> Foo<'a> {
    fn x_value(&self) -> &'a i32 {
        self.x
    }
}
