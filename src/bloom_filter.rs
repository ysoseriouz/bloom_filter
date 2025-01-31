mod bit_array;
mod builder;

use crate::decoder::Decodable;
use crate::encoder::Encodable;
use crate::hash::{fnv, murmur3};
pub use bit_array::BitArray;
pub use builder::BloomFilterBuilder;
use std::fs::File;
use std::io::prelude::*;

const MURMUR3_SEED: u32 = 0xdead_cafe;
const FALSE_POSITIVE_RATE: f32 = 0.01;

#[derive(Debug, PartialEq)]
pub enum CompressMode {
    None,
    Lzw,
}

pub struct BloomFilter {
    pub bit_array: BitArray,
    pub hash_count: usize,
    pub compress_mode: CompressMode,
}

impl BloomFilter {
    pub fn new(max_items: usize) -> Self {
        let ln_rate = FALSE_POSITIVE_RATE.ln();
        let ln_2 = 2_f32.ln();

        let size = (-(max_items as f32) * ln_rate / ln_2.powi(2)).ceil() as usize;
        let hash_count = (-ln_rate / ln_2).ceil() as usize;

        Self {
            bit_array: BitArray::new(size),
            hash_count,
            compress_mode: CompressMode::None,
        }
    }

    pub fn lookup(&self, key: &str) -> bool {
        self.hashing(key).iter().all(|&i| self.bit_array.get_bit(i))
    }

    pub fn insert(&mut self, key: &str) -> bool {
        if self.lookup(key) {
            false
        } else {
            for i in self.hashing(key) {
                self.bit_array.set(i, true)
            }
            true
        }
    }

    // double-hashing
    fn hashing(&self, key: &str) -> Vec<usize> {
        let bitsize = self.bit_array.size;
        let bytes: &[u8] = key.as_bytes();
        let h1 = (murmur3(bytes, MURMUR3_SEED) as usize) % bitsize;
        let h2 = (fnv(bytes) as usize) % bitsize;
        let mut hash_table = vec![0; self.hash_count];

        for (idx, hash_val) in hash_table.iter_mut().enumerate() {
            *hash_val = (h1 + idx.wrapping_mul(h2)) % bitsize
        }

        hash_table
    }

    pub fn to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.encode()).unwrap();
    }

    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        Self::decode(&buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn prepare_tmp_dir() {
        let tmp_dir = Path::new("tmp");

        if !tmp_dir.exists() {
            fs::create_dir(tmp_dir).unwrap();
        }
    }

    #[test]
    fn test_bloom_filter() {
        let mut bloom_filter = BloomFilter::new(100);
        bloom_filter.insert("abound");
        bloom_filter.insert("abound1");
        bloom_filter.insert("abound2");
        bloom_filter.insert("abound");

        assert!(bloom_filter.lookup("abound"));
        assert!(bloom_filter.lookup("abound1"));
        assert!(bloom_filter.lookup("abound2"));
        assert!(!bloom_filter.lookup("aboundd"));
        assert!(!bloom_filter.lookup("abbound"));
        assert!(!bloom_filter.lookup("dnuoba"));
    }

    #[test]
    fn test_bloom_filter_spec() {
        let bloom_filter = BloomFilter::new(2);
        assert_eq!(bloom_filter.bit_array.byte_array.len(), 3);
        assert_eq!(bloom_filter.bit_array.size, 20);
        assert_eq!(bloom_filter.hash_count, 7);
        assert_eq!(bloom_filter.compress_mode, CompressMode::None);
    }

    #[test]
    fn test_persist_local_file() {
        prepare_tmp_dir();
        let test_file = "tmp/bloom_filter_test_persist_local_file.bin";
        let bloom_filter = BloomFilter::new(2);

        // Test no data
        bloom_filter.to_file(test_file);
        assert!(Path::new(test_file).exists());
        let mut bloom_filter = BloomFilter::from_file(test_file);
        assert!(!bloom_filter.lookup("test"));
        assert!(!bloom_filter.lookup("test1"));

        // Test with data
        bloom_filter.insert("test");
        bloom_filter.to_file(test_file);
        assert!(Path::new(test_file).exists());
        let bloom_filter = BloomFilter::from_file(test_file);
        assert!(bloom_filter.lookup("test"));
        assert!(!bloom_filter.lookup("test1"));

        // Cleanup
        fs::remove_file(test_file).unwrap();
    }
}
