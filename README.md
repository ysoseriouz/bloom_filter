# Bloom Filter

Simple bloom filter implementation

## Example

``` rust
use bloom_filter_yss::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new(10); // Number of element to be inserted
    bloom_filter.insert("test");
    bloom_filter.insert("test1");
    bloom_filter.lookup("test");
}
```
