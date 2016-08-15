//! Actual implementation of the Database

use self::path::PathPart;
use self::data::{NodePtr, NodeContent, Data, DataRef};

#[cfg(test)]
mod test;

pub mod data;
pub mod path;

pub struct Database {
    root: NodePtr,
}

impl Database {
    /// Create a new empty data base
    #[allow(new_without_default)]
    pub fn new() -> Database {
        Database {
            root: NodePtr::new(),
        }
    }
}

impl Database {
    /// Resolve one part of the path
    fn resolve_one_part(&self, node: NodePtr, key: DataRef) -> Result<NodePtr, ()> {
        let i_node = node.node();
        let b_i_node = i_node.borrow();

        match *b_i_node.content() {
            NodeContent::Binary(_) => Err(()),
            NodeContent::Dir(ref dir) => match dir.get(key) {
                None => Err(()),
                Some(next) => Ok(next.clone()),
            }
        }
    }

    /// Resolve an entire path and return the resulting node if such path
    /// exists
    fn resolve_path<T: path::IntoPath>(&self, path: T) -> Result<NodePtr, ()> {
        let mut res = Ok(self.root.clone());

        for p in path.into_path().parts() {
            res = match res {
                Err(_) => break,
                Ok(curr) => match *p {
                    PathPart::Binary(ref data) =>
                        self.resolve_one_part(curr, data),
                },
            };
        }

        res
    }

    /// Get a value from the database
    ///
    /// The path you want to get a value from (must be a data)
    ///
    /// return: The data if it exists, and Err otherwise
    pub fn get<T: path::IntoPath>(&self, path: T) -> Result<Data, ()> {
        let node = try!(self.resolve_path(path));
        let i_node = node.node();
        let b_i_node = i_node.borrow();

        match *b_i_node.content() {
            NodeContent::Binary(ref data) => Ok(data.clone()),
            NodeContent::Dir(_) => Err(()),
        }
    }
}
