use std::fmt::Display;

/// Enum representing the different PostgreSQL database schemas used by the
/// application.
///
/// This enum is used to differentiate between different database schemas
/// that are used within the application. Each schema is represented by a
/// unique unsigned 8-bit integer value.
///
/// # Variants
///
/// * `Postgres` - The default PostgreSQL database schema.
/// * `SMDB` - The database schema for the System Manager database.
/// * `CMDB` - The database schema for the Configuration Manager database.
/// * `IMDB` - The database schema for the Integration Manager database.
/// * `MDDB` - The database schema for the Metadata Data database.
///
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PostgresDBSchema {
    /// All postgres schema
    PostgresDBSchemaALL = 255_u8,
    /// The default PostgreSQL database schema.
    #[default]
    PostgresDBSchemaPostgres = 0_u8,
    /// The database schema for the System Manager database.
    PostgresDBSchemaSMDB = 1_u8,
    /// The database schema for the Configuration Manager database.
    PostgresDBSchemaCMDB = 2_u8,
    /// The database schema for the Integration Manager database.
    PostgresDBSchemaIMDB = 3_u8,
    /// The database schema for the Metadata database.
    PostgresDBSchemaMDDB = 4_u8,
}

impl PostgresDBSchema {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(n: u8) -> Option<PostgresDBSchema> {
        match n {
            255 => Some(PostgresDBSchema::PostgresDBSchemaALL),
            0 => Some(PostgresDBSchema::PostgresDBSchemaPostgres),
            1 => Some(PostgresDBSchema::PostgresDBSchemaSMDB),
            2 => Some(PostgresDBSchema::PostgresDBSchemaCMDB),
            3 => Some(PostgresDBSchema::PostgresDBSchemaIMDB),
            4 => Some(PostgresDBSchema::PostgresDBSchemaMDDB),
            _ => None,
        }
    }

    pub fn from_string(n: &str) -> Option<PostgresDBSchema> {
        match n {
            "ALL" => Some(PostgresDBSchema::PostgresDBSchemaALL),
            "Postgres" => Some(PostgresDBSchema::PostgresDBSchemaPostgres),
            "SMDB" => Some(PostgresDBSchema::PostgresDBSchemaSMDB),
            "CMDB" => Some(PostgresDBSchema::PostgresDBSchemaCMDB),
            "IMDB" => Some(PostgresDBSchema::PostgresDBSchemaIMDB),
            "MDDB" => Some(PostgresDBSchema::PostgresDBSchemaMDDB),
            _ => None,
        }
    }
}

impl Display for PostgresDBSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresDBSchema::PostgresDBSchemaALL => write!(f, "ALL"),
            PostgresDBSchema::PostgresDBSchemaPostgres => write!(f, "Postgres"),
            PostgresDBSchema::PostgresDBSchemaSMDB => write!(f, "SMDB"),
            PostgresDBSchema::PostgresDBSchemaCMDB => write!(f, "CMDB"),
            PostgresDBSchema::PostgresDBSchemaIMDB => write!(f, "IMDB"),
            PostgresDBSchema::PostgresDBSchemaMDDB => write!(f, "MDDB"),
        }
    }
}
