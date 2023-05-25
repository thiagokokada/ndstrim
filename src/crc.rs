// Adapted from
// https://github.com/DS-Homebrew/TWiLightMenu/blob/master/booter/animatedbannerpatch.py#L8.
pub fn checksum(data: &[u8]) -> u16 {
    const POLYNOMIAL: u16 = 0xa001;

    let mut crc = 0xffff;
    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            let carry = (crc & 0x1) > 0;
            crc >>= 1;
            if carry {
                crc ^= POLYNOMIAL;
            }
        }
    }

    crc
}