use std::fs;
use std::io::{BufWriter, Write};

pub fn file_io() {
    write_file();
}

fn write_file() {
    let mut f = BufWriter::new(fs::File::create("test.txt").unwrap());
    for _ in 0 .. 100 {
        let b = b"test";
        f.write(b).unwrap();
    }
}
