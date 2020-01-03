use std::io;
use std::io::prelude::*;
use std::fs::File;

#[allow(unused_must_use)]
pub fn read() {
    read_chain();
}

fn read_chain() -> io::Result<()> {
    let c1 = File::open("chain1.txt")?;
    let c2 = File::open("chain2.txt")?;

    let mut chain = c1.chain(c2);
    let mut buffer = String::new();

    chain.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    Ok(())
}
