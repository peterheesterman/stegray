// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

/* 
Tests for:
    new
    save
    to_byte_vector
    from_byte_vector
*/

use super::*;
use std::fs;
use std::path::Path;

#[test]
fn to_byte_vector() {
    let content = vec![
        73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105, 118,
        101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116, 104, 101,
        32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97, 105, 110, 32,
        116, 111, 32, 101, 110, 100, 46,
    ];

    let meta = StegrayMeta {
        file_type: FileType::Text,
        length: 109,
    };

    let stegray = Stegray {
        meta,
        content,
        shasum: String::from("538f5e16ed8412cbb7c2b1e37006e8618f92ff49"),
    };

    let expected_bytes = vec![
        0, 0, 0, 0, 109, 73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32,
        102, 105, 118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32,
        116, 104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
        105, 110, 32, 116, 111, 32, 101, 110, 100, 46, 53, 51, 56, 102, 53, 101, 49, 54, 101, 100,
        56, 52, 49, 50, 99, 98, 98, 55, 99, 50, 98, 49, 101, 51, 55, 48, 48, 54, 101, 56, 54, 49,
        56, 102, 57, 50, 102, 102, 52, 57,
    ];

    assert_eq!(stegray.to_byte_vector(), expected_bytes);
}

#[test]
fn from_byte_vector() {
    let content = vec![
        0, 0, 0, 0, 109, 73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32,
        102, 105, 118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32,
        116, 104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
        105, 110, 32, 116, 111, 32, 101, 110, 100, 46, 53, 51, 56, 102, 53, 101, 49, 54, 101, 100,
        56, 52, 49, 50, 99, 98, 98, 55, 99, 50, 98, 49, 101, 51, 55, 48, 48, 54, 101, 56, 54, 49,
        56, 102, 57, 50, 102, 102, 52, 57,
    ];

    let computed_stegray = Stegray::from_byte_vector(content);

    let expected_content = vec![
        73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105, 118,
        101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116, 104, 101,
        32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97, 105, 110, 32,
        116, 111, 32, 101, 110, 100, 46,
    ];

    let meta = StegrayMeta {
        file_type: FileType::Text,
        length: 109,
    };

    let expected_stegray = Stegray {
        meta,
        content: expected_content,
        shasum: String::from("538f5e16ed8412cbb7c2b1e37006e8618f92ff49"),
    };

    assert_eq!(
        expected_stegray.meta.file_type,
        computed_stegray.meta.file_type
    );
    assert_eq!(expected_stegray.meta.length, computed_stegray.meta.length);
    assert_eq!(expected_stegray.shasum, computed_stegray.shasum);
    assert_eq!(
        expected_stegray.content.len(),
        computed_stegray.content.len()
    );
}

#[test]
fn new_stegray_from_text_file() {
    let text_file = "./src/_test_data/texts/short.txt";
    let stegray = Stegray::new(text_file);

    assert_eq!(stegray.meta.file_type, FileType::Text);
    assert_eq!(stegray.meta.length, 109);
    assert_eq!(
        stegray.content,
        vec![
            73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105,
            118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116,
            104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
            105, 110, 32, 116, 111, 32, 101, 110, 100, 46
        ]
    );
    assert_eq!(stegray.shasum, "538f5e16ed8412cbb7c2b1e37006e8618f92ff49");
}

#[test]
fn save_text_file() {
    let content = vec![
        73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105, 118,
        101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116, 104, 101,
        32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97, 105, 110, 32,
        116, 111, 32, 101, 110, 100, 46,
    ];

    let meta = StegrayMeta {
        file_type: FileType::Text,
        length: 109,
    };

    let stegray = Stegray {
        meta,
        content,
        shasum: String::from("538f5e16ed8412cbb7c2b1e37006e8618f92ff49"),
    };

    let output_path = "./src/_test_data/texts/output/save_text_file.txt";
    stegray.save(output_path);

    assert!(Path::new(output_path).exists());
    fs::remove_file(output_path).expect("remove file failed?");
}
