
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}


impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }

    pub fn prefix(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = self;
        for c in prefix.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node)
    }

    pub fn is_word(&self) -> bool {
        self.is_word
    }

    pub fn clear(&mut self) {
        self.children.clear();
        self.is_word = false;
    }

}

#[cfg(test)]

mod test_trie {
    use super::*;

    #[test]
    fn test_trie_add_string() {
        let mut trie = TrieNode::new();
        trie.insert("hello");
        assert!(trie.prefix("hello").unwrap().is_word());
    }

    #[test]
    fn test_trie_add_strings() {
        let mut trie = TrieNode::new();
        trie.insert("hello");
        trie.insert("world");
        assert!(trie.prefix("hello").unwrap().is_word());
        assert!(trie.prefix("world").unwrap().is_word());
    }
}