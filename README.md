# DataFerret

## Description

DataFerret is a simple key-value store written in Rust. It's designed to be lightweight, fast, and easy to use, making it a great choice for small projects where a full database might be overkill.

![DataFerret Logo](./logo.png)

## Features

- Store data as key-value pairs
- Fast retrieval, update, and deletion of data
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
use data_ferret::DataFerret;
```

Create an instance of DataFerret:

```rust
let mut db = DataFerret::new("./path_to_your_db");
```

Store a key-value pair:

```rust
db.insert("partition_key", "sort_key", "value").unwrap();
```

Fetch a value by key:

```rust
let value = db.fetch("partition_key", "sort_key").unwrap();
```

Update a value:

```rust
db.update("partition_key", "sort_key", "new_value").unwrap();
```

Delete a value:

```rust
db.delete("partition_key", "sort_key").unwrap();
```

## Contributing

If you're interested in contributing, please submit a pull request. All contributions are welcome!

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

If you have any questions, feel free to open an issue.
