pub struct BitArray {
    byte_array: Vec<u8>,
    pub bit_size: usize,
}

impl BitArray {
    pub fn new(bit_size: usize) -> Self {
        let num_bytes = (bit_size + 7) / 8;
        Self {
            byte_array: vec![0; num_bytes],
            bit_size,
        }
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        let (byte_index, bit_offset) = self.get_byte_position(index);
        let mask = 1 << (7 - bit_offset);

        if value {
            self.byte_array[byte_index] |= mask;
        } else {
            self.byte_array[byte_index] &= !mask;
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let (byte_index, bit_offset) = self.get_byte_position(index);
        let mask = 1 << (7 - bit_offset);

        (self.byte_array[byte_index] & mask) != 0
    }

    pub fn get_byte_position(&self, bit_index: usize) -> (usize, usize) {
        let index = bit_index / 8;
        let offset = bit_index % 8;
        (index, offset)
    }
}

impl std::fmt::Display for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bits = vec!['0'; self.bit_size];

        for (index, bit) in bits.iter_mut().enumerate() {
            let (byte_index, bit_offset) = self.get_byte_position(index);
            let bit_value = (self.byte_array[byte_index] >> (7 - bit_offset)) & 1;
            *bit = if bit_value == 1 { '1' } else { '0' };
        }

        write!(f, "{}", bits.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_array() {
        let mut bit_array = BitArray::new(10);
        bit_array.set_bit(5, true);
        bit_array.set_bit(9, true);
        println!("{}", bit_array);
        assert!(bit_array.get_bit(5));
        assert!(bit_array.get_bit(9));
        assert!(!bit_array.get_bit(0));
        bit_array.set_bit(0, true);
        bit_array.set_bit(5, false);
        println!("{}", bit_array);
        assert!(bit_array.get_bit(0));
        assert!(!bit_array.get_bit(5));
    }
}
