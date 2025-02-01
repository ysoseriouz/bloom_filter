use super::Decodable;
use crate::bloom_filter::{BitArray, BloomFilter, CompressMode};
use crate::compressor::lzw;
use crate::error::BloomFilterError;
use anyhow::Result;

impl Decodable for BitArray {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let buffer = &bytes[..8];
        let byte_size = u64::from_be_bytes(buffer.try_into()?) as usize;
        let byte_data = &bytes[8..8 + byte_size];
        let buffer = &bytes[(8 + byte_size)..];
        let size = u64::from_be_bytes(buffer.try_into()?) as usize;

        let bit_array = Self {
            byte_array: byte_data.into(),
            size,
        };
        Ok(bit_array)
    }
}

impl Decodable for CompressMode {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let mode = match bytes[0] {
            1 => Self::Lzw,
            _ => Self::None,
        };
        Ok(mode)
    }
}

impl Decodable for BloomFilter {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let split_idx = bytes.len() - 9;
        let bit_array_buffer = &bytes[..split_idx];
        let buffer = &bytes[split_idx..(bytes.len() - 1)];
        let hash_count = u64::from_be_bytes(buffer.try_into()?) as usize;
        let last_byte = *bytes.last().ok_or(BloomFilterError::InvalidFormat)?;
        let compress_mode = CompressMode::decode(&[last_byte])?;
        let decompressed_bit_array = match compress_mode {
            CompressMode::Lzw => &lzw::decompress(bit_array_buffer),
            _ => bit_array_buffer,
        };

        let bloom_filter = Self {
            bit_array: BitArray::decode(decompressed_bit_array)?,
            hash_count,
            compress_mode,
        };
        Ok(bloom_filter)
    }
}

#[cfg(test)]
mod decodable {
    mod bit_array {
        use crate::bloom_filter::BitArray;
        use crate::decoder::Decodable;

        #[test]
        fn test_decode() {
            let encoded = vec![
                0, 0, 0, 0, 0, 0, 0, 2, // Byte size
                0b00000100, 0b01000000, // Byte data
                0, 0, 0, 0, 0, 0, 0, 10, // Bin size
            ];
            let bit_array = BitArray::decode(&encoded).unwrap();
            assert_eq!(bit_array.size, 10);
            assert!(!bit_array[0]);
            assert!(bit_array[5]);
            assert!(bit_array[9]);
        }
    }

    mod bloom_filter {
        use crate::compressor::lzw;
        use crate::decoder::Decodable;
        use crate::{BloomFilter, CompressMode};

        #[test]
        fn test_decode_without_compression() {
            let encoded = vec![
                0, 0, 0, 0, 0, 0, 0, 3, // BitArray: byte size
                0b10000100, 0b00100001, 0, // BitArray: byte data
                0, 0, 0, 0, 0, 0, 0, 20, // BitArray: bin size
                0, 0, 0, 0, 0, 0, 0, 7, // Number of hash functions
                0, // Compress mode
            ];
            let bloom_filter = BloomFilter::decode(&encoded).unwrap();
            assert_eq!(bloom_filter.bit_array.byte_array.len(), 3);
            assert_eq!(bloom_filter.bit_array.size, 20);
            assert_eq!(bloom_filter.hash_count, 7);
            assert_eq!(bloom_filter.compress_mode, CompressMode::None);
            assert!(bloom_filter.lookup("test"));
            assert!(!bloom_filter.lookup("test1"));
        }

        #[test]
        fn test_decode_with_lzw() {
            let encoded_bit_array = vec![
                0, 0, 0, 0, 0, 0, 0, 3, // BitArray: byte size
                0b10000100, 0b00100001, 0, // BitArray: byte data
                0, 0, 0, 0, 0, 0, 0, 20, // BitArray: bin size
            ];
            let mut encoded = lzw::compress(&encoded_bit_array);
            encoded.extend([
                0, 0, 0, 0, 0, 0, 0, 7, // Number of hash functions
                1, // Compress mode
            ]);

            let bloom_filter = BloomFilter::decode(&encoded).unwrap();
            assert_eq!(bloom_filter.bit_array.byte_array.len(), 3);
            assert_eq!(bloom_filter.bit_array.size, 20);
            assert_eq!(bloom_filter.hash_count, 7);
            assert_eq!(bloom_filter.compress_mode, CompressMode::Lzw);
            assert!(bloom_filter.lookup("test"));
            assert!(!bloom_filter.lookup("test1"));
        }
    }
}
