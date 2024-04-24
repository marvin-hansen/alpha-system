pub fn create_metadata_db() -> String {
    r"
    CREATE DATABASE IF NOT EXISTS metadata
    "
    .to_string()
}

pub fn drop_metadata_db() -> String {
    r"
    DROP DATABASE IF EXISTS metadata
    "
    .to_string()
}
