#![feature(test)]

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct BloomFilter {
    bit_array: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(size: usize, num_hashes: usize) -> Self {
        BloomFilter {
            bit_array: vec![false; size],
            num_hashes,
        }
    }

    pub fn add<T: Hash>(&mut self, item: T) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();

        for i in 0..self.num_hashes {
            let index = (hash + i as u64) % self.bit_array.len() as u64;
            self.bit_array[index as usize] = true;
        }
    }

    pub fn contains<T: Hash>(&self, item: T) -> bool {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();

        for i in 0..self.num_hashes {
            let index = (hash + i as u64) % self.bit_array.len() as u64;
            if !self.bit_array[index as usize] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_add_and_contains() {
        let mut bloom_filter = BloomFilter::new(10000, 3);

        bloom_filter.add("hello");
        bloom_filter.add("world");

        assert!(bloom_filter.contains("hello"));
        assert!(bloom_filter.contains("world"));
        assert!(!bloom_filter.contains("foo"));
    }

    #[test]
    fn test_bloom_filter_false_positive() {
        let mut bloom_filter = BloomFilter::new(10000, 3);
        bloom_filter.add("hello");
        assert!(!bloom_filter.contains("foo"));
    }

    #[test]
    fn test_bloom_filter_no_false_negatives() {
        let mut bloom_filter = BloomFilter::new(10000, 3);
        bloom_filter.add("hello");
        bloom_filter.add("world");
        assert!(bloom_filter.contains("hello"));
        assert!(bloom_filter.contains("world"));
    }
}
