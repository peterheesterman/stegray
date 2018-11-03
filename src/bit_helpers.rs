pub fn transform_u32_to_u8_array(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

pub fn transform_u8_array_to_u32(four_bytes: &[u8]) -> u32 {
      (((*four_bytes)[0] as u32) << 24)
    + (((*four_bytes)[1] as u32) << 16)
    + (((*four_bytes)[2] as u32) << 8)
    + (((*four_bytes)[3] as u32) << 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_u32_to_u8_array_small() {
        assert_eq!(
            transform_u32_to_u8_array(4_u32),
            [0_u8, 0_u8, 0_u8, 4_u8]
        );
    }

    #[test]
    fn transform_u32_to_u8_array_large() {
        assert_eq!(
            transform_u32_to_u8_array(120002004_u32),
            [7_u8, 39_u8, 21_u8, 212_u8]
        );
    }

    #[test]
    fn transform_u8_array_to_u32_large() {
        assert_eq!(
            transform_u8_array_to_u32(&[7_u8, 39_u8, 21_u8, 212_u8]),
            120002004_u32
        );
    }
}
