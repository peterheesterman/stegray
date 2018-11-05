
# ( Work in progress )

# stegray

steg(anography) - array

An intermediate data abstraction for [steg](https://github.com/peterheesterman/steg).

```
pub struct StegrayMeta {
    pub file_type: FileType,
    pub length: u32,
}

// pub fn from_bytes(bytes: [u8; 5]) -> StegrayMeta



pub struct Stegray {
    pub meta: StegrayMeta,
    pub content: Vec<u8>,
    pub shasum: String,
}

// pub fn get_meta_length() -> u32
// pub fn new(path: &str) -> Stegray
// pub fn save(&self, path: &str)
// pub fn to_byte_vector(&self) -> Vec<u8>
// pub fn from_byte_vector(data: Vec<u8>) -> Stegray
```