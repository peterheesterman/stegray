#[allow(unused_imports)]
use super::{get_file_content, get_file_string, get_file_type, get_file_type_from_u8, FileType};

// get_file_type
#[test]
fn get_file_type_png() {
    assert_eq!(get_file_type("image.png"), FileType::PNG);
}

#[test]
fn get_file_type_text() {
    assert_eq!(get_file_type("text.txt"), FileType::Text);
}

#[test]
fn get_file_type_unkown() {
    assert_eq!(get_file_type("text.akflkdsjfa;ksld"), FileType::UNKNOWN);
}

// get_file_string
#[test]
fn get_file_string_short() {
    assert_eq!(
        get_file_string("./src/_test_data/texts/short.txt"),
        "I am first with five\nThen seven in the middle\nFive again to end."
    );
}

// get_file_content
#[test]
fn get_file_content_text_short() {
    assert_eq!(
        get_file_content("./src/_test_data/texts/short.txt"),
        vec![
            73, 32, 97, 109, 32, 102, 105, 114, 115, 116, 32, 119, 105, 116, 104, 32, 102, 105,
            118, 101, 10, 84, 104, 101, 110, 32, 115, 101, 118, 101, 110, 32, 105, 110, 32, 116,
            104, 101, 32, 109, 105, 100, 100, 108, 101, 10, 70, 105, 118, 101, 32, 97, 103, 97,
            105, 110, 32, 116, 111, 32, 101, 110, 100, 46
        ]
    );
}

#[test]
fn get_file_type_from_u8_text() {
    assert_eq!(get_file_type_from_u8(0), FileType::Text);
}

#[test]
fn get_file_type_from_u8_png() {
    assert_eq!(get_file_type_from_u8(1), FileType::PNG);
}

#[test]
fn get_file_type_from_u8_next() {
    assert_eq!(get_file_type_from_u8(2), FileType::UNKNOWN);
}

#[test]
fn get_file_type_from_u8_unknown() {
    assert_eq!(get_file_type_from_u8(255), FileType::UNKNOWN);
}
