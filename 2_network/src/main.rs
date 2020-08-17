
/*
    2_network/src/main.rs
*/

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

//const ZERO: u8 = 0;
/*
const COUNT: u8 = getCount();
fn getCount() -> u8 {
    return 1;
}
*/

// main function
fn main() {
    println!("Starting main... ");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        println!("Connection established!");
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

/* EOF */
