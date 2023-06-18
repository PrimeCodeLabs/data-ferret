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

    pub fn insert(&mut self, key: String, value: String) -> io::Result<()> {
        let data = Data { key: key.clone(), value };
        self.store.insert(key, data.clone());
        self.persistence.save_data(&data)
    }

    pub fn get(&mut self, key: String) -> io::Result<Option<Data>> {
        match self.store.get(&key) {
            Some(data) => Ok(Some(data.clone())),
            None => {
                let data = self.persistence.load_data(key.clone())?;
                self.store.insert(key.clone(), data.clone());
                Ok(Some(data))
            }
        }
    }

    pub fn delete(&mut self, key: String) -> io::Result<()> {
        self.store.delete(&key);
        self.persistence.delete_data(&key)
    }

    // Consider adding this function if you frequently work with the whole dataset
    pub fn load_all_data(&mut self) -> io::Result<()> {
        let data_map = self.persistence.load_all_data()?;
        for (key, value) in data_map {
            self.store.insert(key, value);
        }
        Ok(())
    }
}

