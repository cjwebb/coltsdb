extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SimpleDB {
    data: HashMap<String, String>,
}

// Simple Key-Value store DB.
// Backed by memory, for now... later, persist to disk.
// We're starting simple, REMEMBER!
impl SimpleDB {
    pub fn new() -> SimpleDB {
        SimpleDB {
            data: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Request {
    Insert { key: String, value: String },
    Query { key: String },
}
