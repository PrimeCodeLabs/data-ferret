use data_ferret::db::{Database, Data};
use std::path::PathBuf;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(database_path: &str) -> PathBuf {
        let path = PathBuf::from(database_path);  // specify a directory, not a file
        fs::create_dir_all(&path).unwrap();  // Create directory if it doesn't exist
        println!("Setup complete, test directory created.");
        path
    }
    

    fn teardown(path: PathBuf) {
        fs::remove_dir_all(&path).unwrap();  // Clean up the directory after each test
        println!("Teardown complete, test directory deleted.");
    }


    #[test]
    fn test_insert() {
        println!("Starting test_insert...");
        let test_db_path = "./test_db";
        let path = setup(test_db_path);
        println!("Setup complete, test directory created.");
        let mut database = Database::new(path.clone());
    
        let key = "key".to_string();
        let value = "value".to_string();
    
        println!("Inserting key-value pair into the database...");
        database.insert(key.clone(), value.clone()).unwrap();
    
        // Get the value
        println!("Fetching key-value pair from the database...");
        let result = match database.get(key.clone()) {
            Ok(data) => data,
            Err(e) => {
                println!("Unexpected error: {:?}", e);
                None
            },
        };
    
        assert_eq!(Some(Data { key, value }), result);
        println!("Test_insert completed.");
    
        teardown(path);
    }
    

    #[test]
    fn test_update() {
        println!("Starting test_update...");
        let test_db_path = "./test_db2";
        let path = setup(test_db_path);
        println!("Setup complete, test directory created.");
        let mut database = Database::new(path.clone());
    
        let key = "key".to_string();
        let value = "value".to_string();
        let updated_value = "updated_value".to_string();
    
        // Insert and then update the value
        println!("Inserting key-value pair into the database...");
        database.insert(key.clone(), value.clone()).unwrap();
        println!("Updating key-value pair in the database...");
        database.insert(key.clone(), updated_value.clone()).unwrap();
    
        // Get the updated value
        println!("Fetching updated key-value pair from the database...");
        let result = match database.get(key.clone()) {
            Ok(data) => data,
            Err(e) => {
                println!("Unexpected error: {:?}", e);
                None
            },
        };
    
        println!("Preparing to assert...");
        assert_eq!(Some(Data { key, value: updated_value }), result);
        println!("Assertion complete.");
        
        println!("Test_update completed.");
    
        teardown(path);
    }
    
    #[test]
    fn test_delete() {
        println!("Starting test_delete...");
        let test_db_path = "./test_db3";
        let path = setup(test_db_path);
        println!("Setup complete, test directory created.");
        let mut database = Database::new(path.clone());
    
        let key = "key".to_string();
        let value = "value".to_string();
    
        // Insert and then delete the key
        println!("Inserting key-value pair into the database...");
        database.insert(key.clone(), value.clone()).unwrap();
        println!("Deleting file with key: {}", key);
        match database.delete(key.clone()) {
            Ok(_) => println!("File deleted successfully"),
            Err(e) => println!("Failed to delete file: {:?}", e),
        }
    
        // Try to get the deleted key
        println!("Attempting to fetch deleted key from the database...");
        let result = match database.get(key.clone()) {
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
