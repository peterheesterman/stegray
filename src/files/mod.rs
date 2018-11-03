mod tests;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FileType {
    // Lets be explicit with mapping so that we don't mess it up in future
    Text = 0,
    PNG = 1,
    UNKNOWN = 255,
}

pub fn get_file_type_from_u8(value: u8) -> FileType {
    match value {
        value if value == FileType::Text as u8 => FileType::Text,
        value if value == FileType::PNG as u8 => FileType::PNG,
        value if value == FileType::UNKNOWN as u8 => FileType::UNKNOWN,
        _ => FileType::UNKNOWN,
    }
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
            // TODO: implement png support
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
