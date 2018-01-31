// panic!

pub fn error_handling() {
    // this returns a panic error
    // panic!("crash and burn!");

    // this returns index out of bounds error
    let v = vec![1, 2, 3];
    v[100];
}
