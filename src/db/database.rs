use super::store::Store;
use super::persistence::{Persistence, Data, OperationType};
use std::path::PathBuf;
use std::io;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Database {
    store: Store,
    persistence: Persistence,
    lock: Mutex<()>,
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        Database {
            store: Store::new(),
            persistence: Persistence::new(path),
            lock: Mutex::new(()),
        }
    }


    pub fn get(&mut self, partition_key: String, sort_key: String) -> io::Result<Option<Data>> {
        match self.store.get(&partition_key, &sort_key) {
            Some(data) => Ok(Some(data.clone())),
            None => {
                let data = self.persistence.load_data(partition_key.clone(), sort_key.clone())?;
                self.store.insert(partition_key.clone(), sort_key.clone(), data.clone());
                Ok(Some(data))
            }
        }
    }

    pub fn insert(&mut self, partition_key: String, sort_key: String, value: String) -> io::Result<()> {
        let data = Data { 
            operation_type: OperationType::Insert,
            partition_key: partition_key.clone(), 
            sort_key: sort_key.clone(), 
            value 
        };
            
        // Lock the mutex before modifying the data.
        let _guard = self.lock.lock().unwrap();
            
        self.store.insert(partition_key, sort_key, data.clone());
        self.persistence.save_data(&data)
    }
    
    

    pub fn delete(&mut self, partition_key: String, sort_key: String) -> io::Result<()> {
        self.store.delete(&partition_key, &sort_key);
        self.persistence.delete_data(&partition_key, &sort_key)
    }

    // Consider adding this function if you frequently work with the whole dataset
    pub fn load_all_data(&mut self) -> io::Result<()> {
        let data_map = self.persistence.load_all_data()?;
        for (partition_key, partition) in data_map {
            for (sort_key, value) in partition {
                self.store.insert(partition_key.clone(), sort_key, value);
            }
        }
        Ok(())
    }

    pub fn batch(&mut self, data: Vec<Data>) -> io::Result<()> {
        for item in data {
            match item.operation_type {
                OperationType::Insert | OperationType::Update => {
                    self.insert(item.partition_key, item.sort_key, item.value)?
                },
                OperationType::Delete => {
                    self.delete(item.partition_key, item.sort_key)?
                },
            }
        }
        Ok(())
    }
    

}
