// panic!
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;


pub fn error_handling() {
    // this returns a panic error
    // panic!("crash and burn!");

    // this returns index out of bounds error
    // let v = vec![1, 2, 3];
    // v[100];

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     },
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!(
    //                     "Tried to create file but there was a problem: {:?}",
    //                     e
    //                 )
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!(
    //             "There was a problem opening the file: {:?}",
    //             error
    //         )
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // these 3 ways are the same
    let my_error = read_username_from_file();
    println!("{:?}", my_error);

    let my_error2 = read_username_from_file2();
    println!("{:?}", my_error2);

    let my_error3 = read_username_from_file3();
    println!("{:?}", my_error3);

    // this occurs an error
    // `?` can be used only in function that returns a result
    // let f = File::open("hello.txt")?;
}

// propagating error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // `?` placed after Result value works in the same way as match control
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // `?` placed after Result value works in the same way as match control
    f.read_to_string(&mut s)?;
    Ok(s)
}

// shorten read_username_from_file2()
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
