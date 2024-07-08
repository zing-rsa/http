use std::fs::{remove_file, File};
use std::io;
use std::io::{prelude::*, ErrorKind};

use crate::response;

const RESOURCES_FILE_PATH: &str = "src/resources";

struct RequestDetails<'a> {
    path: &'a str,
    headers: &'a [&'a str],
    data: &'a str,
}

pub fn match_request(message: &str) -> String {
    let req_info = message.split("\r\n\r\n").collect::<Vec<&str>>();
    let info_lines: Vec<&str> = req_info[0].split("\r\n").collect();
    let headers = &info_lines[1..];
    let data = req_info[1];

    match info_lines[0].split(" ").collect::<Vec<&str>>().as_slice() {
        ["GET", path, "HTTP/1.1"] => map_err(
            RequestDetails {
                path,
                headers,
                data,
            },
            &handle_get,
        ),
        ["PUT", path, "HTTP/1.1"] => map_err(
            RequestDetails {
                path,
                headers,
                data,
            },
            &handle_put,
        ),
        ["POST", path, "HTTP/1.1"] => map_err(
            RequestDetails {
                path,
                headers,
                data,
            },
            &handle_post,
        ),
        ["DELETE", path, "HTTP/1.1"] => map_err(
            RequestDetails {
                path,
                headers,
                data,
            },
            &handle_delete,
        ),
        _ => response::method_not_supported(Some("Method not supported")),
    }
}

fn map_err(req: RequestDetails, f: &dyn Fn(RequestDetails) -> io::Result<String>) -> String {
    match f(req) {
        Ok(message) => message,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => response::not_found(Some("Resource not found")),
            _ => response::server_error(Some("Unknown error occurred")),
        },
    }
}

fn handle_get(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{0}", req.path);
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(full_path)?;

    f.read_to_end(&mut buffer)?;

    Ok(response::ok(Some(&buffer)))
}

fn handle_post(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{}", req.path);

    match File::open(&full_path) {
        Ok(_) => Ok(response::bad_request(Some("File already exists"))),
        Err(_) => {
            let mut f = File::create(&full_path)?;

            f.write_all(req.data.as_bytes())?;

            Ok(response::created(None))
        }
    }
}

fn handle_put(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{}", req.path);

    let mut f = File::create(full_path)?;

    f.write_all(req.data.as_bytes())?;

    Ok(response::ok(Some(req.data.as_bytes())))
}

fn handle_delete(req: RequestDetails) -> io::Result<String> {
    let full_path = format!("{RESOURCES_FILE_PATH}{}", req.path);
    remove_file(full_path)?;

    Ok(response::ok(None))
}
