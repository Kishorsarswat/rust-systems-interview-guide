//! Reference Solution: Prefix Tree (Trie)

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNodeSol {
    pub children: HashMap<char, TrieNodeSol>,
    pub is_end_of_word: bool,
}

#[derive(Default, Debug)]
pub struct TrieSol {
    root: TrieNodeSol,
}

impl TrieSol {
    pub fn new() -> Self {
        Self {
            root: TrieNodeSol::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for ch in word.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        curr.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.find_node(word).map_or(false, |node| node.is_end_of_word)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn find_node(&self, prefix: &str) -> Option<&TrieNodeSol> {
        let mut curr = &self.root;
        for ch in prefix.chars() {
            curr = curr.children.get(&ch)?;
        }
        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_trie() {
        let mut trie = TrieSol::new();

        trie.insert("apple");
        assert!(trie.search("apple"));
        assert!(!trie.search("app"));
        assert!(trie.starts_with("app"));

        trie.insert("app");
        assert!(trie.search("app"));
    }
}
