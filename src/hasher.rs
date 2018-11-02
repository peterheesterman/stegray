extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha1::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn get_file_hash(path: &str) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't read {} because: {}", path, why.description()),
        Ok(file) => file,
    };

    let mut data = Vec::new();

    match file.read_to_end(&mut data) {
        Err(why) => panic!("Couldn't read {} because: {}", path, why.description()),
        Ok(_) => {
            let mut sha = Sha1::new();
            sha.input(&data);
            sha.result_str()
        }
    }
}
