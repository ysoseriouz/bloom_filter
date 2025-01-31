# Bloom Filter

Simple bloom filter implementation

## Example

``` rust
use bloom_filter_yss::BloomFilterBuilder;

fn main() {
    let capacity = 100; // Number of element to be inserted
    let local_path = "bloom_filter.bin"

    let mut bloom_filter = BloomFilterBuilder::new(capacity).build();
    bloom_filter.insert("test");
    bloom_filter.insert("test1");
    bloom_filter.lookup("test");

    // Save data to local disk
    bloom_filter.to_file(local_path);

    // Load back to memory
    let bloom_filter = BloomFilterBuilder::load(local_path);
    bloom_filter.lookup("test");
    bloom_filter.lookup("test1");
}
```
