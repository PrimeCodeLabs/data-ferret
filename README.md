# DataFerret

## Description

DataFerret is a simple key-value store written in Rust, designed to store data as partition and sort key pairs. It supports both disk-based and in-memory storage modes, providing flexibility based on your needs. This flexible model allows for more complex and efficient data structures and lookups compared to traditional key-value stores. With DataFerret, you can build high-performance applications that are lightweight and efficient, while leveraging the power and safety of the Rust language.

<div style="text-align:center">
    <img src="./logo.png" alt="DataFerret Logo" width="300"/>
</div>

## Features

- Store data as partition-sort key pairs
- Fast and efficient data retrieval, update, and deletion
- Disk-based and in-memory storage modes
- Batch operation support for efficient multiple data inserts
- Concurrent insert operations for improved performance
- Lightweight design with a focus on performance and simplicity

## Getting Started

### Prerequisites

- Rust 1.51 or higher

### Building

To build the project, navigate to the root of the project directory and run the following command:

```bash
cargo build
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

## Usage

### Importing the Library

Include the following in your Rust code:

```rust
use data_ferret::db::{Database, InMemoryDatabase, DatabaseType};
use data_ferret::db::{Data, OperationType};
```

### Creating a New Database Instance

Specify the path to your database directory and create a new Database instance:

```rust
let path = std::path::PathBuf::from("./my_database_dir");
let mut db: Box<dyn DatabaseType> = Box::new(Database::new(path));
```

Or create a new InMemoryDatabase instance for in-memory operations:

```rust
let mut db: Box<dyn DatabaseType> = Box::new(InMemoryDatabase::new());
```

### Storing Data

Store a key-value pair by providing a partition key, sort key and the associated value:

```rust
let partition_key = "my_partition_key".to_string();
let sort_key = "my_sort_key".to_string();
let value = "my_value".to_string();

db.insert(partition_key.clone(), sort_key.clone(), value.clone());
```

### Retrieving Data

Fetch a value by its partition key and sort key:

```rust
match db.get(partition_key, sort_key) {
    Some(data) => println!("Retrieved data: {:?}", data),
    None => println!("No data found"),
}
```

### Deleting Data

Delete a key-value pair by its partition key and sort key:

```rust
db.delete(partition_key, sort_key);
```

### Batch Operations

Perform batch operations by providing a vector of `Data` objects, each representing a separate operation (This feature is currently supported only for the disk-based `Database` type):

```rust
let data = vec![
    Data { operation_type: OperationType::Insert, partition_key: "partition1".to_string(), sort_key: "sort1".to_string(), value: "value1".to_string() },
    Data { operation_type: OperationType::Insert, partition_key: "partition2".to_string(), sort_key: "sort2".to_string(), value: "value2".to_string() },
];
db.batch(data).unwrap();
```

## Contributing

Contributions are welcome! Feel free to submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

If you have any questions or need further assistance, feel free to open an issue.
