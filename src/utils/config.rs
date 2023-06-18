use std::path::PathBuf;

pub struct Config {
    pub db_path: PathBuf,
    // Add other configuration fields as needed
}

impl Config {
    pub fn new(db_path: PathBuf) -> Self {
        Config { db_path }
    }

    // Add other methods as needed, like loading from a file or environment variables
}
