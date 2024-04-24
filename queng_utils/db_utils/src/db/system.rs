pub fn create_system_db() -> String {
    r"
    CREATE DATABASE IF NOT EXISTS system
    "
    .to_string()
}

pub fn drop_system_db() -> String {
    r"
    DROP DATABASE IF EXISTS system
    "
    .to_string()
}
