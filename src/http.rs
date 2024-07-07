use std::io::prelude::*;
use std::fs::{remove_file, File};
use std::io;

use crate::response;

const RESOURCES_FILE_PATH: &str = "src/resources";

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
