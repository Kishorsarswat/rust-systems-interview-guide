//! # Challenge: Prefix Tree (Trie)
//!
//! ## Problem Statement
//! Implement a Prefix Tree (Trie) for fast string prefix matching and dictionary lookup.
//!
//! Support the following operations:
//! * `insert(&mut self, word: &str)`: Inserts string `word` into the trie.
//! * `search(&self, word: &str) -> bool`: Returns `true` if the exact `word` exists in the trie.
//! * `starts_with(&self, prefix: &str) -> bool`: Returns `true` if any previously inserted string
//!   starts with the given `prefix`.

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end_of_word: bool,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates a new empty `Trie`.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a word into the trie.
    pub fn insert(&mut self, _word: &str) {
        todo!("Implement Trie::insert(&mut self, word)")
    }

    /// Returns `true` if the exact word is present in the trie.
    pub fn search(&self, _word: &str) -> bool {
        todo!("Implement Trie::search(&self, word)")
    }

    /// Returns `true` if there is any word in the trie that starts with the given prefix.
    pub fn starts_with(&self, _prefix: &str) -> bool {
        todo!("Implement Trie::starts_with(&self, prefix)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Implement Trie::insert")]
    fn test_starter_trie_panics_todo() {
        let mut trie = Trie::new();
        trie.insert("rust");
    }

    #[test]
    #[ignore] // Remove #[ignore] once you implement insert, search, and starts_with!
    fn test_trie_basic_ops() {
        let mut trie = Trie::new();

        trie.insert("apple");
        assert!(trie.search("apple"));
        assert!(!trie.search("app"));
        assert!(trie.starts_with("app"));

        trie.insert("app");
        assert!(trie.search("app"));
    }
}
