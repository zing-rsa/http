// use core::slice::SlicePattern;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

const INTERNAL_SERVER_ERROR: &[u8] = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";
const OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

fn handle_get(_path: &str) -> String {
    from_utf8([OK, b"get\r\n"].concat().as_slice()).unwrap().to_string()
}

fn handle_post(_path: &str) -> String {
    from_utf8([OK, b"post\r\n"].concat().as_slice()).unwrap().to_string()
}

fn handle_put(_path: &str) -> String {
    from_utf8([OK, b"put\r\n"].concat().as_slice()).unwrap().to_string()
}

fn handle_delete(_path: &str) -> String {
    from_utf8([OK, b"delete\r\n"].concat().as_slice()).unwrap().to_string()
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ()>{
    println!("new connection!");

    let mut buffer = [0 as u8; 4096];

    match stream.read(&mut buffer) {
        Ok(0) => println!("End of stream!"), // end of stream
        Ok(size) => {
            println!("Got message of size: {}", size);
            print!("{}", from_utf8(&buffer).unwrap());

            let message: &str = from_utf8(&buffer[0..size]).unwrap();
            let lines: Vec<&str> = message.split("\r\n").collect();

            let response = match lines[0].split(" ").collect::<Vec<&str>>().as_slice() {
                ["GET", path, "HTTP/1.1" ] => { handle_get(path) },
                ["POST", path, "HTTP/1.1" ] => { handle_post(path) },
                ["PUT", path, "HTTP/1.1" ] => { handle_put(path) },
                ["DELETE", path, "HTTP/1.1" ] => { handle_delete(path) },

                [verb, _, _] => { 
                    println!("Unknown http verb! {}", verb); 
                    from_utf8(INTERNAL_SERVER_ERROR).unwrap().to_string()
                }
                _ => from_utf8(INTERNAL_SERVER_ERROR).unwrap().to_string()
            };

            stream.write_all(response.as_bytes()).unwrap();
        },
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
        },
        Err(_) => {}
       }
    }

    Ok(())
}

