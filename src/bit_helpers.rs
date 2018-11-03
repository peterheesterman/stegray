pub fn transform_u64_to_u8_array(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}

pub fn transform_u8_array_to_u64(four_bytes: &[u8]) -> u64 {
    (((*four_bytes)[0] as u64) << 56)
        + (((*four_bytes)[1] as u64) << 48)
        + (((*four_bytes)[2] as u64) << 40)
        + (((*four_bytes)[3] as u64) << 32)
        + (((*four_bytes)[4] as u64) << 24)
        + (((*four_bytes)[5] as u64) << 16)
        + (((*four_bytes)[6] as u64) << 8)
        + (((*four_bytes)[7] as u64) << 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_u64_to_u8_array_small() {
        assert_eq!(
            transform_u64_to_u8_array(4_u64),
            [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 4_u8]
        );
    }

    #[test]
    fn transform_u64_to_u8_array_large() {
        assert_eq!(
            transform_u64_to_u8_array(120000000000002004_u64),
            [1_u8, 170_u8, 83_u8, 93_u8, 61_u8, 12_u8, 7_u8, 212_u8]
        );
    }

    #[test]
    fn transform_u8_array_to_u64_large() {
        assert_eq!(
            transform_u8_array_to_u64(&[1_u8, 170_u8, 83_u8, 93_u8, 61_u8, 12_u8, 7_u8, 212_u8]),
            120000000000002004_u64
        );
    }
}
