use data_ferret::db::{InMemoryDatabase, Data, OperationType};

#[cfg(test)]
mod tests {
    use super::*;

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