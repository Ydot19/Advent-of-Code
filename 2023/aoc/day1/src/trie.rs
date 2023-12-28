use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}


impl TrieNode {
    pub(crate) fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }

    pub(crate) fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }

    pub(crate) fn prefix(&self, prefix: &str) -> Option<&TrieNode> {
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



}

#[cfg(test)]

mod test_trie {
    use super::*;

    #[test]
    fn test_trie_add_string() {
        let mut trie = TrieNode::new();
        trie.insert("hello");
    }

    #[test]
    fn test_trie_add_strings() {
        let mut trie = TrieNode::new();
        trie.insert("hello");
        trie.insert("world");
    }
}