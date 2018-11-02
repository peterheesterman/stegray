#[cfg(test)]
mod tests;

mod files;
mod hasher;

use files::FileType;

pub struct Stegray {
    pub file_type: FileType,
    pub length: u64,
    pub content: Vec<u8>,
    pub shasum: String,
}

impl Stegray {
    pub fn new(path: &str) -> Stegray {
        let file_type = files::get_file_type(&path);
        let content = files::get_file_content(&path);

        let meta_length = 40 + 8 + 1; // shasum + length + file_type
        let length = (content.len() + meta_length) as u64;

        Stegray {
            file_type,
            length,
            content,
            shasum: hasher::get_file_hash(path),
        }
    }

    pub fn save(path: &str) {
        println!("Saving file to {}", path);
        // TODO: Write this
    }

    pub fn to_byte_vector(&self) -> Vec<u8> {
        // TODO: Write this
        vec![1]
    }

    pub fn from_byte_vector(_data: Vec<u8>) -> Stegray {
        // TODO: Write this
        Stegray {
            file_type: FileType::Text,
            length: 123,
            content: vec![53, 54, 55],
            shasum: String::from("3b0a64b1ec39cb1d04ebb30858566c653b57a4d4"),
        }
    }
}
