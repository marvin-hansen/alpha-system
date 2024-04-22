pub fn create_system_db() -> String {
    r"
    CREATE DB IF NOT EXISTS system
    "
    .to_string()
}

pub fn drop_system_db() -> String {
    r"
    DROP DB IF EXISTS system
    "
    .to_string()
}
