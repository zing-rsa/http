use std::str::from_utf8;

const OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const INTERNAL_SERVER_ERROR: &[u8] = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";
const NOT_FOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

pub fn ok(payload: &[u8]) -> String {
    from_utf8([OK, payload, b"\r\n"].concat().as_slice()).unwrap().to_string()
}

pub fn not_found(payload: &[u8]) -> String {
    from_utf8([NOT_FOUND, payload, b"\r\n"].concat().as_slice()).unwrap().to_string()
}

pub fn server_error(payload: &[u8]) -> String {
    from_utf8([INTERNAL_SERVER_ERROR, payload, b"\r\n"].concat().as_slice()).unwrap().to_string()
}