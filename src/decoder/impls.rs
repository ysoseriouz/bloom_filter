use super::Decodable;
use crate::bit_array::BitArray;
use crate::compressor::lzw;
use crate::{BloomFilter, CompressMode};

impl Decodable for BitArray {
    fn decode(bytes: &[u8]) -> Self {
        let buffer = &bytes[..8];
        let byte_size = u64::from_be_bytes(buffer.try_into().unwrap()) as usize;
        let byte_data = &bytes[8..8 + byte_size];
        let buffer = &bytes[(8 + byte_size)..];
        let size = u64::from_be_bytes(buffer.try_into().unwrap()) as usize;

        Self {
            byte_array: byte_data.into(),
            size,
        }
    }
}

impl Decodable for CompressMode {
    fn decode(bytes: &[u8]) -> Self {
        match bytes[0] {
            1 => Self::Lzw,
            _ => Self::None,
        }
    }
}

impl Decodable for BloomFilter {
    fn decode(bytes: &[u8]) -> Self {
        let split_idx = bytes.len() - 9;
        let bit_array_buffer = &bytes[..split_idx];
        let buffer = &bytes[split_idx..(bytes.len() - 1)];
        let hash_count = u64::from_be_bytes(buffer.try_into().unwrap()) as usize;
        let compress_mode = CompressMode::decode(&[*bytes.last().unwrap()]);
        let decompressed_bit_array = match compress_mode {
            CompressMode::Lzw => &lzw::decompress(bit_array_buffer),
            _ => bit_array_buffer,
        };

        Self {
            bit_array: BitArray::decode(decompressed_bit_array),
            hash_count,
            compress_mode,
        }
    }
}

#[cfg(test)]
mod decodable {
    mod bit_array {
        use crate::bit_array::BitArray;
        use crate::decoder::Decodable;

        #[test]
        fn test_decode() {
            let encoded = vec![
                0, 0, 0, 0, 0, 0, 0, 2, // Byte size
                0b00000100, 0b01000000, // Byte data
                0, 0, 0, 0, 0, 0, 0, 10, // Bin size
            ];
            let bit_array = BitArray::decode(&encoded);
            assert_eq!(bit_array.size, 10);
            assert!(!bit_array[0]);
            assert!(bit_array[5]);
            assert!(bit_array[9]);
        }
    }

    mod bloom_filter {
        use crate::decoder::Decodable;
        use crate::{BloomFilter, CompressMode};

        #[test]
        fn test_decode() {
            let encoded = vec![
                0, 0, 0, 0, 0, 0, 0, 3, // BitArray: byte size
                0b10000100, 0b00100001, 0, // BitArray: byte data
                0, 0, 0, 0, 0, 0, 0, 20, // BitArray: bin size
                0, 0, 0, 0, 0, 0, 0, 7, // Number of hash functions
                0, // Compress mode
            ];
            let bloom_filter = BloomFilter::decode(&encoded);
            assert_eq!(bloom_filter.bit_array.byte_array.len(), 3);
            assert_eq!(bloom_filter.bit_array.size, 20);
            assert_eq!(bloom_filter.hash_count, 7);
            assert_eq!(bloom_filter.compress_mode, CompressMode::None);
            assert!(bloom_filter.lookup("test"));
            assert!(!bloom_filter.lookup("test1"));
        }
    }
}
