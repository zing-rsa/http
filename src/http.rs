use std::io::prelude::*;
use std::fs::{remove_file, File};
use std::io;
use std::io::{ErrorKind::NotFound};

use crate::response;

const RESOURCES_FILE_PATH: &str = "src/resources";

pub fn match_request(message: &str) -> String {

    let lines: Vec<&str> = message.split("\r\n").collect();

    return match lines[0].split(" ").collect::<Vec<&str>>().as_slice() {
        ["GET", path, "HTTP/1.1"] => match handle_get(path) {
            Ok(data) => data,
            Err(err) => match err.kind() {
                NotFound => response::not_found(None),
                _ => response::server_error(None),
            },
        },
        ["DELETE", path, "HTTP/1.1"] => match handle_delete(path) {
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
    
}

pub fn handle_get(path: &str) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{path}");
    let mut buffer: Vec<u8> = Vec::new();
    
    print!("reading: {}\n", full_path);

    let mut f = File::open(full_path)?;

    f.read_to_end(&mut buffer)?;

    Ok(response::ok(Some(&buffer)))
}

// pub fn handle_post(_path: &str, data: &Vec<u8>) -> io::Result<String> {
//     Ok(response::ok(Some(&buffer)))
// }

// pub fn handle_put(_path: &str, data: &Vec<u8>) -> io::Result<String> {
//     Ok(response::ok(Some(&buffer)))
// }

pub fn handle_delete(path: &str) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{path}");
    remove_file(full_path)?;

    Ok(response::ok(None))
}
