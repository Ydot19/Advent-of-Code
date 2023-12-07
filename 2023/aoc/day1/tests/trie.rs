#![allow(clippy::unwrap_used, clippy::expect_used)]

use day1::trie::TrieNode;

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

