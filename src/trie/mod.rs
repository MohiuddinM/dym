mod node;

use self::node::Node;
use super::pattern::{CharMatcher, Pattern};
use std::slice::Iter;

#[derive(Debug, Eq, PartialEq)]
pub struct Trie {
    root: Node
}

impl Trie {
    
    /// Creates an empty Trie
    pub fn new() -> Trie {
        Trie { root: Node::new() }
    }

    /// Inserts a copy of the given word into the Trie
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for character in word.chars() {
            node = {node}.add_node(character);
        }
        node.is_word = true;
    }

    /// Returns true if the given word is held in the Trie
    pub fn contains(&self, word: &str) -> bool{
        let mut node = &self.root;
        for character in word.chars() {
            match {node}.get_node(character) {
                Some(n) => node = n,
                None => return false,
            };
        }
        node.is_word
    }

    /// Searches the Trie for all words that match the given Pattern
    pub fn all_matches(&self, pattern: &str) -> Vec<String> {
        let pattern = Pattern::from_str(pattern);
        self.recurse_pattern(pattern.iter(), &self.root, String::new())
    }

    /// Recursively searches the Trie until the end of the iterator is reached
    fn recurse_pattern(&self, mut iter: Iter<CharMatcher>, node: &Node, mut trail: String) -> Vec<String> {
        let next = iter.next();

        // Base Case
        if next == None { 
            if node.is_word {
                return vec![trail];
            } else {
                return vec![];
            }
        }

        let matcher = next.unwrap();
        match matcher {
            // descend only into the branch corresponding to char c
            &CharMatcher::Exact(c) => {
                let iter = iter.clone();
                let next_node = node.get_node(c);
                if next_node == None {
                    return vec![];
                }
                let next_node = next_node.unwrap();
                trail.push(c);
                return self.recurse_pattern(iter, next_node, trail);
            },
            // descend into all branches from this node
            &CharMatcher::Any => {
                let mut words = Vec::new();
                for (c, node) in node.nodes.iter() {
                    let iter = iter.clone();
                    let next_node = node;
                    let mut trail = trail.clone();
                    trail.push(*c);
                    words.extend_from_slice(self.recurse_pattern(iter, next_node, trail).as_slice());
                }
                return words;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_pattern() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("cut");
        trie.insert("cot");
        trie.insert("dog");
        trie.insert("car");

        let matches = trie.all_matches("c*t");

        assert!(matches.contains(&"cat".to_string()));
        assert!(matches.contains(&"cut".to_string()));
        assert!(matches.contains(&"cot".to_string()));
        
        assert!(!matches.contains(&"dog".to_string()));
        assert!(!matches.contains(&"car".to_string()));
    }
}
