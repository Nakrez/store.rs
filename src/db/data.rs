//! Definitions of the internal data types

use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

/// Internal type of the raw data stored
pub type Data = Vec<u8>;

/// A reference over an internal data
pub type DataRef<'a> = &'a [u8];

/// Internal type of the external directory
pub type Directory = HashMap<Data, NodePtr>;

/// Actual content of the node
pub enum NodeContent {
    /// The node contains a binary
    Binary(Data),
    /// The node contains a directory (HashMap)
    Dir(Directory)
}

/// Internal node of the data base
pub struct Node {
    content: NodeContent,
}

impl Node {
    /// Create a new node of the data base
    #[allow(new_without_default)]
    pub fn new() -> Self {
        Node {
            content: NodeContent::Dir(Directory::new()),
        }
    }

    /// Returns the content of the node
    pub fn content(&self) -> &NodeContent {
        &self.content
    }

    /// Returns a mutable reference to the content of the node
    pub fn content_mut(&mut self) -> &mut NodeContent {
        &mut self.content
    }
}

/// A shareable pointer to a Node
#[derive(Clone)]
pub struct NodePtr(Arc<RefCell<Node>>);

impl NodePtr {
    /// Create a new shareable pointer to a new node
    #[allow(new_without_default)]
    pub fn new() -> Self {
        NodePtr(Arc::new(RefCell::new(Node::new())))
    }

    /// Get the internal node
    pub fn node(&self) -> Arc<RefCell<Node>> {
        self.0.clone()
    }
}
