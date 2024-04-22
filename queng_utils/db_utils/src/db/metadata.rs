pub fn create_metadata_db() -> String {
    r"
    CREATE DB IF NOT EXISTS metadata
    "
    .to_string()
}

pub fn drop_metadata_db() -> String {
    r"
    DROP DB IF EXISTS metadata
    "
    .to_string()
}
