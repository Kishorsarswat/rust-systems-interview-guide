//! Reference Solution: $O(1)$ LRU Cache using Index-based Doubly Linked Array

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCacheSol<K, V> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    free_head: Option<usize>,
    head: Option<usize>, // Most Recently Used (MRU)
    tail: Option<usize>, // Least Recently Used (LRU)
}

impl<K: Eq + Hash + Clone, V> LruCacheSol<K, V> {
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

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&idx) = self.map.get(key) {
            self.move_to_head(idx);
            Some(&self.nodes[idx].val)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].val = value;
            self.move_to_head(idx);
            return;
        }

        if self.map.len() >= self.capacity {
            self.evict_lru();
        }

        let idx = if let Some(free_idx) = self.free_head {
            self.free_head = self.nodes[free_idx].next;
            self.nodes[free_idx] = Node {
                key: key.clone(),
                val: value,
                prev: None,
                next: None,
            };
            free_idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key: key.clone(),
                val: value,
                prev: None,
                next: None,
            });
            idx
        };

        self.push_head(idx);
        self.map.insert(key, idx);
    }

    fn move_to_head(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }

        self.detach(idx);
        self.push_head(idx);
    }

    fn detach(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }

        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            self.tail = prev;
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = None;
    }

    fn push_head(&mut self, idx: usize) {
        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        } else {
            self.tail = Some(idx);
        }

        self.head = Some(idx);
    }

    fn evict_lru(&mut self) {
        if let Some(lru_idx) = self.tail {
            let key_to_remove = self.nodes[lru_idx].key.clone();
            self.map.remove(&key_to_remove);

            self.detach(lru_idx);

            self.nodes[lru_idx].next = self.free_head;
            self.free_head = Some(lru_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_lru_cache() {
        let mut cache = LruCacheSol::new(2);

        cache.put(1, "A");
        cache.put(2, "B");
        assert_eq!(cache.get(&1), Some(&"A"));

        cache.put(3, "C"); // 2 evicted
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"C"));

        cache.put(4, "D"); // 1 evicted
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&3), Some(&"C"));
        assert_eq!(cache.get(&4), Some(&"D"));
    }
}
