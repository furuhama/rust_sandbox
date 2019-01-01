// http server

use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;

pub fn echo_server() {
    println!("staring echo server");
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

fn server_start() -> io::Result<()> {
    // use localhost:8080
    // `?` at the end of this line, means simple error handling.
    let lis = TcpListener::bind("127.0.0.1:8080")?;

    for stream in lis.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occurred while accepting a connection: {}", e);
                continue;
            }
        };

        let _ = thread::spawn(move || -> io::Result<()> {
            loop {
                let mut b = [0; 1024];
                let n = stream.read(&mut b)?;
                if n == 0 {
                    return Ok(());
                } else {
                    stream.write_all(&b[0..n])?;
                }
            }
        });
    }
    Ok(())
}
