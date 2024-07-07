use std::io::{prelude::*, ErrorKind};
use std::fs::{remove_file, File};
use std::io;
use std::io::{ErrorKind::NotFound};
use std::collections::HashMap;

use crate::response;

const RESOURCES_FILE_PATH: &str = "src/resources";

struct RequestDetails<'a> {
    path: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    data: Option<Vec<u8>>
}

pub fn match_request(message: &str) -> String {
    let lines: Vec<&str> = message.split("\r\n").collect();

    let res: String = match lines[0].split(" ").collect::<Vec<&str>>().as_slice() {
        ["GET", path, "HTTP/1.1"] => map_err(RequestDetails {path: path, headers: None, data: None }, &handle_get),
        ["DELETE", path, "HTTP/1.1"] => map_err(RequestDetails { path: path, headers: None, data: None }, &handle_delete),
        // ["POST", path, "HTTP/1.1"] => map_err(RequestDetails { path: path, headers: None, data: None }, &handle_delete),
        _ => response::bad_request(None)
    };

    res


    // let req: Option<Request> = match lines[0].split(" ").collect::<Vec<&str>>().as_slice() {
    //     [method, path, "HTTP/1.1"] => Some(Request {method: method, path: path, headers: None, data: None }),
    //     _ => None
    // };

    // match req {
    //     Some(Request { method: "GET", path, .. }) => map_err(path, &handle_get),
    //     Some(Request { method: "DELETE", path, .. }) => map_err(path, &handle_delete),
    //     // Request { method: "GET", path, headers, data } => handle_get(path),
    //     // Request { method: "GET", path, headers, data } => handle_get(path),
    //     _ => response::bad_request(None)
    // }
}

fn map_err(req: RequestDetails, f: &dyn Fn(RequestDetails) -> io::Result<String>) -> String {
    match f(req) {
        Ok(message) => message,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => response::not_found(None),
            _ => response::server_error(None)
        }  
    }
}

fn handle_get(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{0}", req.path);
    let mut buffer: Vec<u8> = Vec::new();
    
    print!("reading: {}\n", full_path);

    let mut f = File::open(full_path)?;

    f.read_to_end(&mut buffer)?;

    Ok(response::ok(Some(&buffer)))
}

// fn handle_post(_path: &str, data: &Vec<u8>) -> io::Result<String> {
//     Ok(response::ok(Some(&buffer)))
// }

// fn handle_put(_path: &str, data: &Vec<u8>) -> io::Result<String> {
//     Ok(response::ok(Some(&buffer)))
// }

fn handle_delete(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{}", req.path);
    remove_file(full_path)?;

    Ok(response::ok(None))
}
