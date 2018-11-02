mod tests;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Text,
    PNG,
    UNKNOWN,
}

pub fn get_file_content(path: &str) -> Vec<u8> {
    let file_type = get_file_type(&path);
    let mut contents: Vec<u8>;

    match file_type {
        FileType::Text => {
            let payload = get_file_string(path);
            let bytes = payload.as_bytes();
            contents = Vec::with_capacity(bytes.len() as usize);
            contents.extend_from_slice(bytes);
        }
        FileType::PNG => {
            // TODO: Write this
            panic!("PNG support coming soon");
        }
        FileType::UNKNOWN => panic!("Bad file type!"),
    };

    contents
}

pub fn get_file_string<'a>(path: &'a str) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file because: {}", why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file to a string");

    contents
}

pub fn get_file_type(path: &str) -> FileType {
    if path.ends_with(".txt") {
        FileType::Text
    } else if path.ends_with(".png") {
        FileType::PNG
    } else {
        FileType::UNKNOWN
    }
}
