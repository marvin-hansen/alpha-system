use common_database::prelude::PostgresDBSchema;

#[test]
fn test_from_u8() {
    assert_eq!(
        PostgresDBSchema::from_u8(0),
        Some(PostgresDBSchema::PostgresDBSchemaPostgres)
    );
    assert_eq!(
        PostgresDBSchema::from_u8(1),
        Some(PostgresDBSchema::PostgresDBSchemaSMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_u8(2),
        Some(PostgresDBSchema::PostgresDBSchemaCMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_u8(3),
        Some(PostgresDBSchema::PostgresDBSchemaIMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_u8(4),
        Some(PostgresDBSchema::PostgresDBSchemaMDDB)
    );
    assert_eq!(PostgresDBSchema::from_u8(5), None);
}

#[test]
fn test_from_string() {
    assert_eq!(
        PostgresDBSchema::from_string("Postgres"),
        Some(PostgresDBSchema::PostgresDBSchemaPostgres)
    );
    assert_eq!(
        PostgresDBSchema::from_string("SMDB"),
        Some(PostgresDBSchema::PostgresDBSchemaSMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("CMDB"),
        Some(PostgresDBSchema::PostgresDBSchemaCMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("IMDB"),
        Some(PostgresDBSchema::PostgresDBSchemaIMDB)
    );
    assert_eq!(
        PostgresDBSchema::from_string("MDDB"),
        Some(PostgresDBSchema::PostgresDBSchemaMDDB)
    );
    assert_eq!(PostgresDBSchema::from_string("DB"), None);
}

#[test]
fn test_as_u8() {
    assert_eq!(PostgresDBSchema::PostgresDBSchemaPostgres.as_u8(), 0);
    assert_eq!(PostgresDBSchema::PostgresDBSchemaSMDB.as_u8(), 1);
    assert_eq!(PostgresDBSchema::PostgresDBSchemaCMDB.as_u8(), 2);
    assert_eq!(PostgresDBSchema::PostgresDBSchemaIMDB.as_u8(), 3);
    assert_eq!(PostgresDBSchema::PostgresDBSchemaMDDB.as_u8(), 4);
}

#[test]
fn test_display() {
    assert_eq!(
        format!("{}", PostgresDBSchema::PostgresDBSchemaPostgres),
        "Postgres"
    );
    assert_eq!(
        format!("{}", PostgresDBSchema::PostgresDBSchemaSMDB),
        "SMDB"
    );
    assert_eq!(
        format!("{}", PostgresDBSchema::PostgresDBSchemaCMDB),
        "CMDB"
    );
    assert_eq!(
        format!("{}", PostgresDBSchema::PostgresDBSchemaIMDB),
        "IMDB"
    );
    assert_eq!(
        format!("{}", PostgresDBSchema::PostgresDBSchemaMDDB),
        "MDDB"
    );
}
