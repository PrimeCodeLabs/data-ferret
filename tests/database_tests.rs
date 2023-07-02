use data_ferret::db::{Database, Data, OperationType};
use std::path::PathBuf;
use std::fs;

#[cfg(test)]
mod tests {

    use std::{thread, sync::{Arc, Mutex}};

    use data_ferret::db::InMemoryDatabase;

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
        assert_eq!(Some(Data {operation_type:OperationType::Insert, partition_key, sort_key, value }), result);

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
        assert_eq!(Some(Data { operation_type: OperationType::Insert, partition_key, sort_key, value: updated_value }), result);
    
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
    
    #[test]
    fn test_concurrent_insert() {
        let path = setup("./test_db4");
        let database = Database::new(path.clone());
        let database = Arc::new(Mutex::new(database));
        let mut handles = vec![];
    
        for i in 0..10 {
            let database = Arc::clone(&database);
            let handle = thread::spawn(move || {
                let partition_key = format!("partition{}", i);
                let sort_key = format!("sort{}", i);
                let value = format!("value{}", i);
                database.lock().unwrap().insert(partition_key, sort_key, value).unwrap();
            });
            handles.push(handle);
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
    
        let mut database = Arc::try_unwrap(database).unwrap().into_inner().unwrap();
        
        for i in 0..10 {
            let partition_key = format!("partition{}", i);
            let sort_key = format!("sort{}", i);
            let result = database.get(partition_key.clone(), sort_key.clone()).unwrap();
            assert_eq!(Some(Data {operation_type: OperationType::Insert, partition_key, sort_key, value: format!("value{}", i) }), result);
        }
        
        teardown(path);
    }

    #[test]
    fn test_batch() {
        let path = setup("./test_db5");
        let mut database = Database::new(path.clone());
    
        let data = vec![
            Data { 
                operation_type: OperationType::Insert,
                partition_key: "partition1".to_string(), 
                sort_key: "sort1".to_string(), 
                value: "value1".to_string() 
            },
            Data { 
                operation_type: OperationType::Insert,
                partition_key: "partition2".to_string(), 
                sort_key: "sort2".to_string(), 
                value: "value2".to_string() 
            },
        ];
    
        database.batch(data).unwrap();
    
        let result1 = database.get("partition1".to_string(), "sort1".to_string()).unwrap();
        assert_eq!(Some(Data { operation_type: OperationType::Insert, partition_key: "partition1".to_string(), sort_key: "sort1".to_string(), value: "value1".to_string() }), result1);
    
        let result2 = database.get("partition2".to_string(), "sort2".to_string()).unwrap();
        assert_eq!(Some(Data { operation_type: OperationType::Insert, partition_key: "partition2".to_string(), sort_key: "sort2".to_string(), value: "value2".to_string() }), result2);
    
        teardown(path);
    }

    #[test]
    fn test_insert_nonexistent_key() {
        let path = setup("./test_db6");
        let mut database = Database::new(path.clone());

        let partition_key = "nonexistent".to_string();
        let sort_key = "nonexistent".to_string();
        let value = "value".to_string();

        let result = database.insert(partition_key.clone(), sort_key.clone(), value.clone());
        assert!(result.is_ok(), "Failed to insert value for nonexistent key");

        teardown(path);
    }

    #[test]
    fn test_concurrent_update() {
        let path = setup("./test_db7");
        let database = Database::new(path.clone());
        let database = Arc::new(Mutex::new(database));
        let mut handles = vec![];

        for _ in 0..10 {
            let database = Arc::clone(&database);
            let handle = thread::spawn(move || {
                let partition_key = "concurrent".to_string();
                let sort_key = "concurrent".to_string();
                let value = format!("value{}", rand::random::<u32>());
                database.lock().unwrap().insert(partition_key, sort_key, value).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut database = Arc::try_unwrap(database).unwrap().into_inner().unwrap();
        let result = database.get("concurrent".to_string(), "concurrent".to_string()).unwrap();
        assert!(result.is_some(), "Failed to get value after concurrent updates");

        teardown(path);
    }

    #[test]
    fn test_insert_empty_value() {
        let path = setup("./test_db8");
        let mut database = Database::new(path.clone());

        let partition_key = "empty".to_string();
        let sort_key = "empty".to_string();
        let value = "".to_string();

        let result = database.insert(partition_key.clone(), sort_key.clone(), value.clone());
        assert!(result.is_ok(), "Failed to insert empty value");

        let result = database.get(partition_key.clone(), sort_key.clone()).unwrap();
        assert_eq!(Some(Data { operation_type: OperationType::Insert, partition_key, sort_key, value }), result);

        teardown(path);
    }

    #[test]
    fn test_insert_in_memory() {
        let mut database = InMemoryDatabase::new();

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone());

        let result = database.get(partition_key.clone(), sort_key.clone());
        assert_eq!(Some(Data {operation_type:OperationType::Insert, partition_key, sort_key, value }), result);
    }

    #[test]
    fn test_update_in_memory() {
        let mut database = InMemoryDatabase::new();
    
        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();
        let updated_value = "updated_value".to_string();
    
        database.insert(partition_key.clone(), sort_key.clone(), value.clone());
        database.insert(partition_key.clone(), sort_key.clone(), updated_value.clone());
    
        let result = database.get(partition_key.clone(), sort_key.clone());
        assert_eq!(Some(Data { operation_type: OperationType::Insert, partition_key, sort_key, value: updated_value }), result);
    }

    #[test]
    fn test_delete_in_memory() {
        let mut database = InMemoryDatabase::new();

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone());
        database.delete(partition_key.clone(), sort_key.clone());

        let result = database.get(partition_key.clone(), sort_key.clone());
        assert_eq!(None, result);
    }

    #[test]
    fn test_get_in_memory() {
        let mut database = InMemoryDatabase::new();

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone());

        let result = database.get(partition_key.clone(), sort_key.clone());
        assert_eq!(Some(Data {operation_type:OperationType::Insert, partition_key, sort_key, value }), result);
    }

    #[test]
    fn test_get_all_in_memory() {
        let mut database = InMemoryDatabase::new();

        let partition_key = "partition".to_string();
        let sort_key = "sort".to_string();
        let value = "value".to_string();

        database.insert(partition_key.clone(), sort_key.clone(), value.clone());

        let result = database.get_all(partition_key.clone());
        println!("{:?}", result);
    }
}
