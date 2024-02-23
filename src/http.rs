use crate::response;

pub fn handle_get(_path: &str) -> String {
    response::ok(b"get")
}

pub fn handle_post(_path: &str) -> String {
    response::ok(b"post")
}

pub fn handle_put(_path: &str) -> String {
    response::ok(b"put")
}

pub fn handle_delete(_path: &str) -> String {
    response::ok(b"delete")
}
