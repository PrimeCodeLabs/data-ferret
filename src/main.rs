use std::io;
use std::env;
use std::path::PathBuf;

use data_ferret::db::Database;

fn main() {
    // Use the first command-line argument for the database file
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from(env::current_dir().expect("Failed to get current dir"))
    };

    let mut db = Database::new(path);

    loop {
        println!("1. Insert data");
        println!("2. Retrieve data");
        println!("3. Delete data");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                let (partition_key, sort_key, value) = get_data_from_user();
                db.insert(partition_key, sort_key, value).expect("Failed to insert data");
            },
            Ok(2) => {
                let (partition_key, sort_key) = get_keys_from_user();
                match db.get(partition_key, sort_key) {
                    Ok(Some(data)) => println!("Retrieved data: {:?}", data),
                    Ok(None) => println!("No data found"),
                    Err(e) => println!("Failed to get data: {}", e),
                }
            },
            Ok(3) => {
                let (partition_key, sort_key) = get_keys_from_user();
                match db.delete(partition_key, sort_key) {
                    Ok(_) => println!("Data deleted successfully"),
                    Err(e) => println!("Failed to delete data: {}", e),
                }
            },
            Ok(4) => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn get_data_from_user() -> (String, String, String) {
    let mut partition_key = String::new();
    println!("Enter a partition key:");
    io::stdin().read_line(&mut partition_key).expect("Failed to read line");

    let mut sort_key = String::new();
    println!("Enter a sort key:");
    io::stdin().read_line(&mut sort_key).expect("Failed to read line");

    let mut value = String::new();
    println!("Enter a value:");
    io::stdin().read_line(&mut value).expect("Failed to read line");

    (partition_key.trim().into(), sort_key.trim().into(), value.trim().into())
}

fn get_keys_from_user() -> (String, String) {
    let mut partition_key = String::new();
    println!("Enter a partition key:");
    io::stdin().read_line(&mut partition_key).expect("Failed to read line");

    let mut sort_key = String::new();
    println!("Enter a sort key:");
    io::stdin().read_line(&mut sort_key).expect("Failed to read line");

    (partition_key.trim().into(), sort_key.trim().into())
}