use super::Encodable;
use crate::bit_array::BitArray;
use crate::BloomFilter;

impl Encodable for BitArray {
    fn byte_size(&self) -> usize {
        8 + self.byte_array.len() + 8
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(self.byte_size());

        encoded.extend_from_slice(&(self.byte_array.len() as u64).to_be_bytes());
        encoded.extend_from_slice(&self.byte_array);
        encoded.extend_from_slice(&(self.size as u64).to_be_bytes());

        encoded
    }
}

impl Encodable for BloomFilter {
    fn byte_size(&self) -> usize {
        self.bit_array.byte_size() + 8
    }

    fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(self.byte_size());

        encoded.extend_from_slice(&self.bit_array.encode());
        encoded.extend_from_slice(&(self.hash_count as u64).to_be_bytes());

        encoded
    }
}

#[cfg(test)]
mod encodable {
    mod bit_array {
        use crate::bit_array::BitArray;
        use crate::encoder::Encodable;

        #[test]
        fn test_byte_size() {
            let bit_array = BitArray::new(10);
            assert_eq!(bit_array.byte_size(), 18);
        }

        #[test]
        fn test_encode() {
            let mut bit_array = BitArray::new(10);
            assert_eq!(
                bit_array.encode(),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 2, // Byte size
                    0, 0, // Byte data
                    0, 0, 0, 0, 0, 0, 0, 10, // Bin size
                ]
            );

            bit_array.set(5, true);
            bit_array.set(9, true);
            assert_eq!(
                bit_array.encode(),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 2, // Byte size
                    0b00000100, 0b01000000, // Byte data
                    0, 0, 0, 0, 0, 0, 0, 10, // Bin size
                ]
            );
        }
    }

    mod bloom_filter {
        use crate::encoder::Encodable;
        use crate::BloomFilter;

        #[test]
        fn test_byte_size() {
            let bloom_filter = BloomFilter::new(2);
            assert_eq!(bloom_filter.byte_size(), 8 + 3 + 8 + 8);
        }

        #[test]
        fn test_encode() {
            let mut bloom_filter = BloomFilter::new(2);
            assert_eq!(
                bloom_filter.encode(),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 3, // BitArray: byte size
                    0, 0, 0, // BitArray: byte data
                    0, 0, 0, 0, 0, 0, 0, 20, // BitArray: bin size
                    0, 0, 0, 0, 0, 0, 0, 7, // Number of hash functions
                ]
            );

            bloom_filter.insert("test");
            assert_eq!(
                bloom_filter.encode(),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 3, // BitArray: byte size
                    0b10000100, 0b00100001, 0, // BitArray: byte data
                    0, 0, 0, 0, 0, 0, 0, 20, // BitArray: bin size
                    0, 0, 0, 0, 0, 0, 0, 7, // Number of hash functions
                ]
            );
        }
    }
}
