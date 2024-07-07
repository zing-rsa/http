// use core::slice::SlicePattern;
use std::io::{ErrorKind::NotFound, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

mod http;
mod response;

fn handle_connection(mut stream: TcpStream) -> Result<(), ()> {
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
                ["GET", path, "HTTP/1.1"] => match http::handle_get(path) {
                    Ok(data) => data,
                    Err(err) => match err.kind() {
                        NotFound => response::not_found(None),
                        _ => response::server_error(None),
                    },
                },
                ["DELETE", path, "HTTP/1.1"] => match http::handle_delete(path) {
                    Ok(data) => data,
                    Err(err) => match err.kind() {
                        NotFound => response::not_found(None),
                        _ => response::server_error(None)
                    }
                },
                // ["POST", path, "HTTP/1.1"] => match http::handle_post(path) {
                //     Ok(data) => data,
                //     Err(err) => match err.kind() {
                //         NotFound => response::not_found(None),
                //         _ => response::server_error(None),
                //     },
                // },
                // ["PUT", path, "HTTP/1.1"] => match http::handle_put(path) {
                //     Ok(data) => data,
                //     Err(err) => match err.kind() {
                //         NotFound => response::not_found(None),
                //         _ => response::server_error(None)
                //     }
                // },
                _ => response::server_error(None),
            };

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
