use super::bit_array::BitArray;
use super::hash::{fnv, murmur3};

const MURMUR3_SEED: u32 = 0xdead_cafe;

pub struct BloomFilter {
    bit_array: BitArray,
    hash_count: usize,
}

impl BloomFilter {
    pub fn new(max_items: usize) -> Self {
        const FALSE_POSITIVE_RATE: f32 = 0.01;
        let ln_rate = FALSE_POSITIVE_RATE.ln();
        let ln_2 = 2_f32.ln();

        let size = (-(max_items as f32) * ln_rate / ln_2.powi(2)).ceil() as usize;
        let hash_count = (-ln_rate / ln_2).ceil() as usize;

        Self {
            bit_array: BitArray::new(size),
            hash_count,
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
