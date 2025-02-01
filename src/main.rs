use bloom_filter_yss::{BloomFilter, BloomFilterBuilder};

fn lookup(bloom_filter: &BloomFilter, key: &str) {
    if bloom_filter.lookup(key) {
        println!("{} may exist", key);
    } else {
        println!("{} good!!!!!", key);
    }
}

fn main() {
    let local_path = "tmp/bloom_filter.bin";
    let mut bloom_filter = BloomFilterBuilder::new(10_000).build();
    bloom_filter.insert("test");
    bloom_filter.insert("test1");
    bloom_filter.to_file(local_path);

    let bloom_filter = BloomFilterBuilder::load(local_path).unwrap();
    lookup(&bloom_filter, "test");
    lookup(&bloom_filter, "test1");
    lookup(&bloom_filter, "test2");
}
