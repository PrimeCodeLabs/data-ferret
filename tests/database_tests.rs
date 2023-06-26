use data_ferret::db::{Database, Data};
use std::path::PathBuf;
use std::fs;

#[cfg(test)]
mod tests {

    use super::*;

    fn setup(database_path: &str) -> PathBuf {
        let path = PathBuf::from(database_path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn teardown(path: PathBuf) {
        fs::remove_dir_all(&path).unwrap();
    }

    #[test]
    fn test_insert() {
        let path = setup("./test_db");
        let mut database = Database::new(path.clone());

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone()).unwrap();

        let result = database.get(partition_key.clone(), sort_key.clone()).unwrap();
        assert_eq!(Some(Data { partition_key, sort_key, value }), result);

        teardown(path);
    }

    #[test]
    fn test_update() {
        let path = setup("./test_db2");
        let mut database = Database::new(path.clone());

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();
        let updated_value = "updated_value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone()).unwrap();
        database.insert(partition_key.clone(), sort_key.clone(), updated_value.clone()).unwrap();

        let result = database.get(partition_key.clone(), sort_key.clone()).unwrap();
        assert_eq!(Some(Data { partition_key, sort_key, value: updated_value }), result);

        teardown(path);
    }

    #[test]
    fn test_delete() {
        println!("Starting test_delete...");
        let test_db_path = "./test_db3";
        let path = setup(test_db_path);
        println!("Setup complete, test directory created.");
        let mut database = Database::new(path.clone());
    
        let partition_key = "key".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();
    
        let file_path = path.join(&partition_key).join(&sort_key);
    
        // Insert key-value pair
        println!("Inserting key-value pair into the database...");
        database.insert(partition_key.clone(), sort_key.clone(), value.clone()).unwrap();
        
        // Check that the file does exist after insert operation
        assert!(file_path.exists(), "File does not exist after insert operation.");
    
        // Delete the key
        println!("Deleting file with key: {}", partition_key);
        match database.delete(partition_key.clone(), sort_key.clone()) {
            Ok(_) => println!("File deleted successfully"),
            Err(e) => println!("Failed to delete file: {:?}", e),
        }
    
        // Check that the file does not exist after delete operation
        assert!(!file_path.exists(), "File still exists after delete operation.");
    
        // Try to get the deleted key
        println!("Attempting to fetch deleted key from the database...");
        let result = match database.get(partition_key.clone(), sort_key.clone()) {
            Ok(result) => result,
            Err(e) => {
                println!("Expected error: {:?}", e);
                None
            },
        };
    
        assert_eq!(None, result);
        println!("Test_delete completed.");
    
        teardown(path);
    }
    
}
