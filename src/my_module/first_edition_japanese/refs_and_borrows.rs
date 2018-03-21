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
}
