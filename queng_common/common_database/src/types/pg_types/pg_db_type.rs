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
    ALL = 255_u8,
    /// The default PostgreSQL database schema.
    #[default]
    Postgres = 0_u8,
    /// The database schema for the System Manager database.
    SMDB = 1_u8,
    /// The database schema for the Configuration Manager database.
    CMDB = 2_u8,
    /// The database schema for the Integration Manager database.
    IMDB = 3_u8,
    /// The database schema for the Metadata database.
    MDDB = 4_u8,
}

impl PostgresDBSchema {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(n: u8) -> Option<PostgresDBSchema> {
        match n {
            255 => Some(PostgresDBSchema::ALL),
            0 => Some(PostgresDBSchema::Postgres),
            1 => Some(PostgresDBSchema::SMDB),
            2 => Some(PostgresDBSchema::CMDB),
            3 => Some(PostgresDBSchema::IMDB),
            4 => Some(PostgresDBSchema::MDDB),
            _ => None,
        }
    }

    pub fn from_string(n: &str) -> Option<PostgresDBSchema> {
        match n {
            "ALL" => Some(PostgresDBSchema::ALL),
            "Postgres" => Some(PostgresDBSchema::Postgres),
            "SMDB" => Some(PostgresDBSchema::SMDB),
            "CMDB" => Some(PostgresDBSchema::CMDB),
            "IMDB" => Some(PostgresDBSchema::IMDB),
            "MDDB" => Some(PostgresDBSchema::MDDB),
            _ => None,
        }
    }
}

impl Display for PostgresDBSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresDBSchema::ALL => write!(f, "ALL"),
            PostgresDBSchema::Postgres => write!(f, "Postgres"),
            PostgresDBSchema::SMDB => write!(f, "SMDB"),
            PostgresDBSchema::CMDB => write!(f, "CMDB"),
            PostgresDBSchema::IMDB => write!(f, "IMDB"),
            PostgresDBSchema::MDDB => write!(f, "MDDB"),
        }
    }
}
