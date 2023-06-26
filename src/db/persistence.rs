use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub partition_key: String,
    pub sort_key: String,
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
        let partition_path = self.path.join(&data.partition_key);
        if !partition_path.exists() {
            fs::create_dir_all(&partition_path)?;
        }
        
        let mut file = File::create(partition_path.join(&data.sort_key))?;
        let data_string = serde_json::to_string(data)?;
        write!(file, "{}", data_string)?;
        Ok(())
    }

    pub fn load_data(&self, partition_key: String, sort_key: String) -> io::Result<Data> {
        let mut file = File::open(self.path.join(&partition_key).join(&sort_key))?;
        let mut data_string = String::new();
        file.read_to_string(&mut data_string)?;
        let data: Data = serde_json::from_str(&data_string)?;
        Ok(data)
    }

    pub fn delete_data(&self, partition_key: &String, sort_key: &String) -> io::Result<()> {
        let filename = format!("{}/{}", partition_key, sort_key);
        let path = self.path.join(&filename);
        println!("Trying to delete file at path: {:?}", path);
        fs::remove_file(path)?;
        Ok(())
    }
    
    

    // Add a new method to load all data from disk into the store.
    pub fn load_all_data(&self) -> io::Result<HashMap<String, HashMap<String, Data>>> {
        let mut data_map = HashMap::new();

        for partition_entry in fs::read_dir(&self.path)? {
            let partition_entry = partition_entry?;
            let partition_path = partition_entry.path();
            if partition_path.is_dir() {
                if let Some(partition_key) = partition_path.file_name() {
                    let partition_key = partition_key.to_string_lossy().into_owned();
                    let mut partition = HashMap::new();
                    for sort_entry in fs::read_dir(partition_path)? {
                        let sort_entry = sort_entry?;
                        let sort_path = sort_entry.path();
                        if sort_path.is_file() {
                            if let Some(sort_key) = sort_path.file_stem() {
                                let sort_key = sort_key.to_string_lossy().into_owned();
                                let data = self.load_data(partition_key.clone(), sort_key.clone())?;
                                partition.insert(sort_key, data);
                            }
                        }
                    }
                    data_map.insert(partition_key, partition);
                }
            }
        }

        Ok(data_map)
    }
}
