use common_database::prelude::PostgresDBSchema;

#[test]
fn test_from_u8() {
    assert_eq!(
        PostgresDBSchema::from_u8(0),
        Some(PostgresDBSchema::Postgres)
    );
    assert_eq!(PostgresDBSchema::from_u8(1), Some(PostgresDBSchema::SMDB));
    assert_eq!(PostgresDBSchema::from_u8(2), Some(PostgresDBSchema::CMDB));
    assert_eq!(PostgresDBSchema::from_u8(3), Some(PostgresDBSchema::IMDB));
    assert_eq!(PostgresDBSchema::from_u8(4), Some(PostgresDBSchema::MDDB));
    assert_eq!(PostgresDBSchema::from_u8(5), None);
}

#[test]
fn test_from_string() {
    assert_eq!(
        PostgresDBSchema::from_string("Postgres"),
        Some(PostgresDBSchema::Postgres)
    );
    assert_eq!(
        PostgresDBSchema::from_string("SMDB"),
        Some(PostgresDBSchema::SMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("CMDB"),
        Some(PostgresDBSchema::CMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("IMDB"),
        Some(PostgresDBSchema::IMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("MDDB"),
        Some(PostgresDBSchema::MDDB)
    );
    assert_eq!(PostgresDBSchema::from_string("DB"), None);
}

#[test]
fn test_as_u8() {
    assert_eq!(PostgresDBSchema::Postgres.as_u8(), 0);
    assert_eq!(PostgresDBSchema::SMDB.as_u8(), 1);
    assert_eq!(PostgresDBSchema::CMDB.as_u8(), 2);
    assert_eq!(PostgresDBSchema::IMDB.as_u8(), 3);
    assert_eq!(PostgresDBSchema::MDDB.as_u8(), 4);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", PostgresDBSchema::Postgres), "Postgres");
    assert_eq!(format!("{}", PostgresDBSchema::SMDB), "SMDB");
    assert_eq!(format!("{}", PostgresDBSchema::CMDB), "CMDB");
    assert_eq!(format!("{}", PostgresDBSchema::IMDB), "IMDB");
    assert_eq!(format!("{}", PostgresDBSchema::MDDB), "MDDB");
}
