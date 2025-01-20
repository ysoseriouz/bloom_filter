// ref: https://en.wikipedia.org/wiki/MurmurHash

use std::io::prelude::*;

const BYTE_CHUNK: usize = 4;

fn murmur_32_scramble(k: u32) -> u32 {
    k.wrapping_mul(0xcc9e2d51)
        .rotate_left(15)
        .wrapping_mul(0x1b873593)
}

pub fn murmur3(data: &[u8], seed: u32) -> u32 {
    let mut hash = seed;
    let mut bytes = data;
    let mut buffer = [0; BYTE_CHUNK];
    let mut len = 0;

    while let Ok(bytes_read) = bytes.read(&mut buffer) {
        match bytes_read {
            BYTE_CHUNK => {
                len += BYTE_CHUNK;
                let k = u32::from_le_bytes(buffer);
                hash ^= murmur_32_scramble(k);
                hash = hash.rotate_left(13);
                hash = hash.wrapping_mul(5).wrapping_add(0xe6546b64);
            }
            1..BYTE_CHUNK => {
                len += bytes_read;
                // Pad with zeros
                for byte in &mut buffer[bytes_read..] {
                    *byte = 0;
                }
                let k = u32::from_le_bytes(buffer);
                hash ^= murmur_32_scramble(k);
            }
            0 => {
                hash ^= len as u32;
                hash ^= hash >> 16;
                hash = hash.wrapping_mul(0x85ebca6b);
                hash ^= hash >> 13;
                hash = hash.wrapping_mul(0xc2b2ae35);
                hash ^= hash >> 16;
                break;
            }
            _ => panic!("buffer read error"),
        }
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur3() {
        assert_eq!(murmur3(b"", 0x00000000), 0x00000000);
        assert_eq!(murmur3(b"", 0x00000001), 0x514e28b7);
        assert_eq!(murmur3(b"", 0xffffffff), 0x81f16f39);
        assert_eq!(murmur3(b"test", 0x00000000), 0xba6bd213);
        assert_eq!(murmur3(b"test", 0x9747b28c), 0x704b81dc);
        assert_eq!(murmur3(b"Hello, world!", 0x00000000), 0xc0363e43);
        assert_eq!(murmur3(b"Hello, world!", 0x9747b28c), 0x24884cba);
        assert_eq!(
            murmur3(b"The quick brown fox jumps over the lazy dog", 0x00000000),
            0x2e4ff723
        );
        assert_eq!(
            murmur3(b"The quick brown fox jumps over the lazy dog", 0x9747b28c),
            0x2fa826cd
        );
    }
}
