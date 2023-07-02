use std::collections::HashMap;
use super::persistence::Data;

#[derive(Debug)]
pub struct Store {
    data: HashMap<String, HashMap<String, Data>>,
}


impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, partition_key: String, sort_key: String, value: Data) {
        let partition = self.data.entry(partition_key).or_insert_with(HashMap::new);
        partition.insert(sort_key, value);
    }
    

    pub fn get(&self, partition_key: &String, sort_key: &String) -> Option<&Data> {
        self.data.get(partition_key).and_then(|partition| partition.get(sort_key))
    }

    pub fn get_all(&self, partition_key: &String) -> Option<&HashMap<String, Data>> {
        self.data.get(partition_key)
    }
    

    pub fn delete(&mut self, partition_key: &String, sort_key: &String) {
        if let Some(partition) = self.data.get_mut(partition_key) {
            partition.remove(sort_key);
        }
    }
    
    pub fn load_all(&mut self, data: HashMap<String, HashMap<String, Data>>) {
        self.data = data;
    }    
}
