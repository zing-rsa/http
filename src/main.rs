mod http;
mod response;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use http::match_request;

fn handle_connection(mut stream: TcpStream) -> Result<(), ()> {
    println!("new connection!");

    let mut buffer = [0 as u8; 4096];

    match stream.read(&mut buffer) {
        Ok(0) => println!("End of stream!"), // end of stream
        Ok(size) => {
            println!("Got message of size: {}", size);
            print!("{}\n", from_utf8(&buffer).unwrap());

            let message: &str = from_utf8(&buffer[0..size]).unwrap();

            let response = match_request(message);

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {
            println!("{}", "Error!");
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).unwrap();
            }
            Err(_) => {}
        }
    }

    Ok(())
}
