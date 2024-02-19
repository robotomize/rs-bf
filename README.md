# bf
This project provides a simple and efficient implementation of a Bloom Filter in the Rust programming language. A Bloom Filter is a data structure that allows for fast membership queries in a set with minimal memory usage.

## Why
This is a learning project, I'm trying to use rust.

## Features
* Ease of Use: A straightforward and easy-to-understand API for adding and checking elements.
* Efficiency: Uses minimal memory, making it ideal for large datasets.
* Testing: Includes a suite of tests to ensure correct operation.

## Installation
Add the following dependency to your Cargo.toml:
```toml
[dependencies]
rust-bloom-filter = "0.1.0"

```
## Example Usage

```rust
use rust_bloom_filter::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new(10000,   3);
    
    bloom_filter.add("hello");
    bloom_filter.add("world");
    
    println!("Contains 'hello': {}", bloom_filter.contains("hello")); 
    println!("Contains 'world': {}", bloom_filter.contains("world")); 
    println!("Contains 'foo': {}", bloom_filter.contains("foo")); 
}

```

