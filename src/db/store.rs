use std::collections::HashMap;
use super::persistence::Data;

pub struct Store {
    data: HashMap<String, Data>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Data) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&Data> {
        self.data.get(key)
    }

    // Delete a key-value pair
    pub fn delete(&mut self, key: &String) {
        self.data.remove(key);
    }

    // Load all data into the store
    pub fn load_all(&mut self, data: HashMap<String, Data>) {
        self.data = data;
    }
}
