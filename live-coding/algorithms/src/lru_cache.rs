//! # Challenge: Least Recently Used (LRU) Cache
//!
//! ## Problem Statement
//! Design and implement a data structure for a Least Recently Used (LRU) cache.
//! It should support the following operations in **$O(1)$ average time complexity**:
//!
//! * `get(&mut self, key: &K) -> Option<&V>`: Retrieve the value associated with the key if present,
//!   marking the item as the Most Recently Used (MRU). Returns `None` if the key is not in the cache.
//! * `put(&mut self, key: K, value: V)`: Insert or update a key-value pair. If the insertion causes
//!   the number of keys to exceed `capacity`, evict the Least Recently Used (LRU) key.
//!
//! ## Constraints & Notes
//! * Avoid using third-party crates.
//! * Standard `std::collections::HashMap` combined with an index-based doubly-linked list (`Vec<Node>`)
//!   or raw pointer indirection is a common idiomatic approach in Rust to avoid borrow checker conflicts.

use std::collections::HashMap;
use std::hash::Hash;

/// Node inside internal doubly-linked array list
#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    prev: Option<usize>,
    next: Option<usize>,
}

/// LRU Cache implementation
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    free_head: Option<usize>,
    head: Option<usize>, // Most Recently Used (MRU)
    tail: Option<usize>, // Least Recently Used (LRU)
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// Constructs a new `LruCache` with a fixed positive `capacity`.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
            free_head: None,
            head: None,
            tail: None,
        }
    }

    /// Fetches a reference to the value associated with `key`, marking it as MRU.
    pub fn get(&mut self, _key: &K) -> Option<&V> {
        todo!("Implement LruCache::get(&mut self, key)")
    }

    /// Inserts or updates key-value pair, evicting the LRU element if capacity is reached.
    pub fn put(&mut self, _key: K, _value: V) {
        todo!("Implement LruCache::put(&mut self, key, value)")
    }

    /// Returns current number of items stored in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Implement LruCache::put")]
    fn test_starter_put_panics_todo() {
        let mut cache = LruCache::new(2);
        cache.put(1, "one");
    }

    #[test]
    #[ignore] // Remove #[ignore] once you implement get and put to verify your solution!
    fn test_lru_cache_basic_ops() {
        let mut cache = LruCache::new(2);

        cache.put(1, "A");
        cache.put(2, "B");
        assert_eq!(cache.get(&1), Some(&"A")); // 1 becomes MRU

        cache.put(3, "C"); // 2 is evicted (LRU)
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"C"));

        cache.put(4, "D"); // 1 is evicted (LRU)
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&3), Some(&"C"));
        assert_eq!(cache.get(&4), Some(&"D"));
    }
}
