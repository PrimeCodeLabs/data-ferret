use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub key: String,
    pub value: String,
}

pub struct Persistence {
    path: PathBuf,
}

impl Persistence {
    pub fn new(path: PathBuf) -> Self {
        // If the directory doesn't exist, create it
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create directory");
        }
    
        Persistence { path }
    }
    


    pub fn save_data(&self, data: &Data) -> io::Result<()> {
        let mut file = File::create(self.path.join(&data.key))?;

        let data_string = serde_json::to_string(data)?;

        write!(file, "{}", data_string)?;

        Ok(())
    }

    pub fn load_data(&self, key: String) -> io::Result<Data> {
        let mut file = File::open(self.path.join(&key))?;
        let mut data_string = String::new();

        file.read_to_string(&mut data_string)?;

        let data: Data = serde_json::from_str(&data_string)?;

        Ok(data)
    }

    pub fn delete_data(&self, key: &String) -> io::Result<()> {
        fs::remove_file(self.path.join(key))?;
        Ok(())
    }

    // Add a new method to load all data from disk into the store.
    pub fn load_all_data(&self) -> io::Result<HashMap<String, Data>> {
        let mut data_map = HashMap::new();

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(key) = path.file_stem() {
                    let key = key.to_string_lossy().into_owned();
                    let data = self.load_data(key.clone())?;
                    data_map.insert(key, data);
                }
            }
        }

        Ok(data_map)
    }
}
