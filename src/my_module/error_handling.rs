// panic!

pub fn error_handling() {
    use std::fs::File;
    use std::io::ErrorKind;

    // this returns a panic error
    // panic!("crash and burn!");

    // this returns index out of bounds error
    // let v = vec![1, 2, 3];
    // v[100];

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
