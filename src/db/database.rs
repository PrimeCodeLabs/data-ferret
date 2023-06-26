use super::store::Store;
use super::persistence::{Persistence, Data};
use std::path::PathBuf;
use std::io;

pub struct Database {
    store: Store,
    persistence: Persistence,
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        Database {
            store: Store::new(),
            persistence: Persistence::new(path),
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
        let data = Data { partition_key: partition_key.clone(), sort_key: sort_key.clone(), value };
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
}
