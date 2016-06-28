use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub nodes: HashMap<char, Box<Node>>,
    pub is_word: bool
}

impl Node {

    /// Creates a new Node with is_word set to false by default
    pub fn new() -> Self {
        Node { nodes: HashMap::new(), is_word: false }
    }

    /// If a node does not already exist for the given character,
    /// creates a new one and returns a reference to it.
    /// If one already exists, returns a reference to the existing node.
    pub fn add_node(&mut self, ch: char) -> &mut Node {
        self.nodes.entry(ch).or_insert(Box::new(Node::new()))
    }

    /// Returns a reference to the node held at the given letter.
    pub fn get_node<'a>(&'a self, ch: char) -> Option<&'a Node> {
        self.nodes.get(&ch).map(|node| &**node)
    }
}
