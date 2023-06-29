# DataFerret

## Description

DataFerret is a simple key-value store written in Rust, designed to store data as partition and sort key pairs. It's designed to be lightweight, fast, and easy to use, making it a great choice for small to medium-sized projects where a full database might be overkill. With features like batch operations and concurrent inserts, it offers performance and functionality while maintaining simplicity.

<div style="text-align:center">
    <img src="./logo.png" alt="DataFerret Logo" width="600"/>
</div>

## Features

- Store data as partition-sort key pairs
- Fast retrieval, update, and deletion of data
- Supports batch operations for efficient multiple data inserts
- Concurrent insert operations for improved performance
- Lightweight and efficient
- Written in Rust

## Getting Started

### Prerequisites

- Rust 1.51 or higher

### Building

To build the project, run the following command in the root of the project directory:

```bash
cargo build
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

## Usage

Include the following in your Rust code:

```rust
use data_ferret::db::Database;
use data_ferret::db::{Data, OperationType};
```

Create an instance of Database:

```rust
let mut db = Database::new("./path_to_your_db");
```

Store a key-value pair:

```rust
db.insert("partition_key", "sort_key", "value").unwrap();
```

Fetch a value by key:

```rust
let value = db.get("partition_key", "sort_key").unwrap();
```

Update a value:

```rust
db.insert("partition_key", "sort_key", "new_value").unwrap();
```

Delete a value:

```rust
db.delete("partition_key", "sort_key").unwrap();
```

Perform a batch operation:

```rust
let data = vec![
    Data { operation_type: OperationType::Insert, partition_key: "partition1".to_string(), sort_key: "sort1".to_string(), value: "value1".to_string() },
    Data { operation_type: OperationType::Insert, partition_key: "partition2".to_string(), sort_key: "sort2".to_string(), value: "value2".to_string() },
];
db.batch(data).unwrap();
```

## Contributing

If you're interested in contributing, please submit a pull request. All contributions are welcome!

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

If you have any questions, feel free to open an issue.
