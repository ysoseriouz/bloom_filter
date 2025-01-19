use super::bit_array::BitArray;

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

    pub fn lookup(&self, s: &str) -> bool {
        self.hashing(s).iter().all(|&i| self.bit_array.get_bit(i))
    }

    pub fn insert(&mut self, s: &str) {
        if self.lookup(s) {
            println!("{} is probably present", s);
        } else {
            for i in self.hashing(s) {
                self.bit_array.set(i, true)
            }
            println!("Inserted: {}", s);
        }
    }

    fn hashing(&self, s: &str) -> Vec<usize> {
        vec![
            hash1(s, self.bit_array.size),
            hash2(s, self.bit_array.size),
            hash3(s, self.bit_array.size),
            hash4(s, self.bit_array.size),
        ]
    }
}

fn mod_exp(base: usize, exp: usize, modulo: usize) -> usize {
    let mut result = 1_usize;
    let mut base = base % modulo;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        base = (base * base) % modulo;
        exp /= 2;
    }

    result
}

fn hash1(s: &str, size: usize) -> usize {
    let mut hash = 0;
    for ch in s.chars() {
        let ch_val = ch as usize;
        hash = (hash + ch_val) % size;
    }
    hash
}

fn hash2(s: &str, size: usize) -> usize {
    let mut hash = 1;
    for (idx, ch) in s.chars().enumerate() {
        let power = mod_exp(19, idx, size);
        let temp = power * (ch as usize) % size;
        hash = (hash + temp) % size;
    }
    hash
}

fn hash3(s: &str, size: usize) -> usize {
    let mut hash = 7;
    for ch in s.chars() {
        hash = (hash * 31 + (ch as usize)) % size;
    }
    hash
}

fn hash4(s: &str, size: usize) -> usize {
    let mut hash = 3;
    let p = 7;
    let fst_ch_val = s.chars().next().unwrap() as usize;
    for idx in 0..s.len() {
        let temp = mod_exp(p, idx, size) * fst_ch_val % size;
        hash = (hash * p + temp) % size;
    }
    hash
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
