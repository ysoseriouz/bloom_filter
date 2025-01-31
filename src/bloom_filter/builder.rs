use super::{BloomFilter, CompressMode};

pub struct BloomFilterBuilder {
    bloom_filter: BloomFilter,
}

impl BloomFilterBuilder {
    pub fn new(capacity: usize) -> Self {
        let mut bloom_filter = BloomFilter::new(capacity);
        bloom_filter.compress_mode = CompressMode::Lzw;

        Self { bloom_filter }
    }

    pub fn no_compress(mut self) -> Self {
        self.bloom_filter.compress_mode = CompressMode::None;
        self
    }

    pub fn build(self) -> BloomFilter {
        self.bloom_filter
    }

    pub fn load(path: &str) -> BloomFilter {
        BloomFilter::from_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let bloom_filter = BloomFilterBuilder::new(100).build();
        assert_eq!(bloom_filter.compress_mode, CompressMode::Lzw);
    }

    #[test]
    fn test_no_compression() {
        let bloom_filter = BloomFilterBuilder::new(100).no_compress().build();
        assert_eq!(bloom_filter.compress_mode, CompressMode::None);
    }
}
