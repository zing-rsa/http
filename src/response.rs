use std::str::from_utf8;


const OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const INTERNAL_SERVER_ERROR: &[u8] = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";
const NOT_FOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

pub fn ok(payload: Option<&[u8]>) -> String {
    println!("200: OK\n");
    from_utf8([OK, payload.unwrap_or(b""), b"\r\n"].concat().as_slice()).unwrap().to_string()
}

pub fn not_found(payload: Option<&[u8]>) -> String {
    println!("404: Not found\n");
    from_utf8([NOT_FOUND, payload.unwrap_or(b""), b"\r\n"].concat().as_slice()).unwrap().to_string()
}

pub fn server_error(payload: Option<&[u8]>) -> String {
    println!("500: Server Error\n");
    from_utf8([INTERNAL_SERVER_ERROR, payload.unwrap_or(b""), b"\r\n"].concat().as_slice()).unwrap().to_string()
}