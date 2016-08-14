//! Actual implementation of the Database

use std::collections::HashMap;

pub type Data = Vec<u8>;

struct Node {
    data: Data,
}

pub struct Database {
    data: HashMap<Data, Node>,
}

impl Database {
    pub fn new(_root_pwd: Data) -> Self {
        Database {
            data: HashMap::new(),
        }
    }
}
