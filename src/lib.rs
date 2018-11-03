#[cfg(test)]
mod tests;

mod bit_helpers;
mod files;
mod hasher;

use std::fs;
use std::str;

use files::FileType;

#[derive(Debug)]
pub struct StegrayMeta {
    pub file_type: FileType,
    pub length: u32,
}

#[derive(Debug)]
pub struct Stegray {
    pub meta: StegrayMeta,
    pub content: Vec<u8>,
    pub shasum: String,
}

impl Stegray {
    pub fn get_meta_length() -> u32 {
        40 + 4 + 1 // shasum + length + file_type
    }

    pub fn new(path: &str) -> Stegray {
        let file_type = files::get_file_type(&path);
        let content = files::get_file_content(&path);

        let length = content.len() as u32 + Stegray::get_meta_length();

        let meta = StegrayMeta { file_type, length };

        Stegray {
            meta,
            content,
            shasum: hasher::get_file_hash(path),
        }
    }

    pub fn save(&self, path: &str) {
        match self.meta.file_type {
            FileType::Text => {
                let content = String::from(str::from_utf8(&self.content).unwrap());
                fs::write(path, content).expect("Unable to write text file.");
            }
            FileType::PNG => {
                // TODO: implement png support
                panic!("Unimplemented");
            }
            FileType::UNKNOWN => {
                panic!("Unknon file type to save!");
            }
        }
        println!("Saved file to {}", path);
    }

    pub fn to_byte_vector(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        data.push(self.meta.file_type as u8);
        data.extend_from_slice(&bit_helpers::transform_u32_to_u8_array(self.meta.length));
        data.extend_from_slice(self.content.as_slice());
        data.extend_from_slice(self.shasum.as_bytes());

        data
    }

    pub fn from_byte_vector(data: Vec<u8>) -> Stegray {
        let file_type = files::get_file_type_from_u8(data[0]);
        let offset = 5;
        let length = bit_helpers::transform_u8_array_to_u32(&data[1..offset]);

        let content_length = length as usize - Stegray::get_meta_length() as usize;

        let mut content = Vec::new();

        match data.get(offset..content_length + offset) {
            Some(buff) => content.extend_from_slice(buff),
            _ => panic!("Data could not be vectorized"),
        }

        let shasum = String::from(str::from_utf8(&data[content_length + offset..]).unwrap());

        let meta = StegrayMeta { file_type, length };

        Stegray {
            meta,
            content,
            shasum: shasum,
        }
    }
}
