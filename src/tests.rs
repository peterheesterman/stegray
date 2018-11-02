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

#[test]
fn new_text_file() {
    let text_file = "./src/_test_data/texts/short.txt";
    let stegray = Stegray::new(text_file);

    assert_eq!(stegray.file_type, FileType::Text);
    assert_eq!(stegray.length, 113);
    assert_eq!(stegray.content, vec![
            73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105,
            118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116,
            104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
            105, 110, 32, 116, 111, 32, 101, 110, 100, 46
        ]);
    assert_eq!(stegray.shasum, "538f5e16ed8412cbb7c2b1e37006e8618f92ff49");
}

#[test]
fn save_text_file() {
    let content = vec![
            73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105,
            118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116,
            104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
            105, 110, 32, 116, 111, 32, 101, 110, 100, 46
        ];

    let stegray = Stegray {
        file_type: FileType::Text,
        length: 113,
        content,
        shasum: String::from("538f5e16ed8412cbb7c2b1e37006e8618f92ff49")
    };
    
    let output_path = "./src/_test_data/texts/output/save_text_file.txt";
    stegray.save(output_path);
}