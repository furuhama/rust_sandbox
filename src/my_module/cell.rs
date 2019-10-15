use std::cell::Cell;

pub fn cell() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;

    x.set(2);
    dbg!(&x);
    y.set(3);
    dbg!(&x);
    z.set(4);
    dbg!(&x);

    println!("{}", x.get());
}
