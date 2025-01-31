/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::fmt::Display;

/// Enum representing the different `PostgreSQL` database schemas used by the
/// application.
///
/// This enum is used to differentiate between different database schemas
/// that are used within the application. Each schema is represented by a
/// unique unsigned 8-bit integer value.
///
/// # Variants
///
/// * `Postgres` - The default `PostgreSQL` database schema.
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
    /// The default `PostgreSQL` database schema.
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
    #[must_use]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    #[must_use]
    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            255 => Some(Self::PostgresDBSchemaALL),
            0 => Some(Self::PostgresDBSchemaPostgres),
            1 => Some(Self::PostgresDBSchemaSMDB),
            2 => Some(Self::PostgresDBSchemaCMDB),
            3 => Some(Self::PostgresDBSchemaIMDB),
            4 => Some(Self::PostgresDBSchemaMDDB),
            _ => None,
        }
    }

    #[must_use]
    pub fn from_string(n: &str) -> Option<Self> {
        match n {
            "ALL" => Some(Self::PostgresDBSchemaALL),
            "Postgres" => Some(Self::PostgresDBSchemaPostgres),
            "SMDB" => Some(Self::PostgresDBSchemaSMDB),
            "CMDB" => Some(Self::PostgresDBSchemaCMDB),
            "IMDB" => Some(Self::PostgresDBSchemaIMDB),
            "MDDB" => Some(Self::PostgresDBSchemaMDDB),
            _ => None,
        }
    }
}

impl Display for PostgresDBSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PostgresDBSchemaALL => write!(f, "ALL"),
            Self::PostgresDBSchemaPostgres => write!(f, "Postgres"),
            Self::PostgresDBSchemaSMDB => write!(f, "SMDB"),
            Self::PostgresDBSchemaCMDB => write!(f, "CMDB"),
            Self::PostgresDBSchemaIMDB => write!(f, "IMDB"),
            Self::PostgresDBSchemaMDDB => write!(f, "MDDB"),
        }
    }
}
