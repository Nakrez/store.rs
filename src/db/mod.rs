//! Actual implementation of the Database

use self::path::PathPart;
use self::data::{NodePtr};

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

    pub fn get<T: path::IntoPath>(&self, key: T) {
        let _path = key.into_path();
    }
}
