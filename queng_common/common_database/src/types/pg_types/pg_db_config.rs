use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// Configuration for the Postgres database.
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct PostgresDBConfig {
    /// DNS resolvable name of the host to connect to.
    pg_host: String,
    /// Postgres username.
    pg_user: String,
    /// Postgres password.
    pg_password: String,
    /// Postgres database name.
    pg_database: String,
    /// Postgres server address.
    pg_port: u16,
    /// Postgres max connections.
    pg_max_connections: u32,
}

impl PostgresDBConfig {
    #[must_use]
    pub const fn new(
        pg_host: String,
        pg_user: String,
        pg_password: String,
        pg_database: String,
        pg_port: u16,
        pg_max_connections: u32,
    ) -> Self {
        Self {
            pg_host,
            pg_user,
            pg_password,
            pg_database,
            pg_port,
            pg_max_connections,
        }
    }
}

impl Default for PostgresDBConfig {
    fn default() -> Self {
        Self::new(
            "localhost".to_string(),
            "postgres".to_string(),
            "postgres".to_string(),
            "test".to_string(),
            5432,
            5,
        )
    }
}

impl PostgresDBConfig {
    /// `PostgresSQL` connection string from the `DBConfig`.
    ///
    #[must_use]
    pub fn pg_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_user, self.pg_password, self.pg_host, self.pg_port, self.pg_database,
        )
    }

    #[must_use]
    pub fn tsn(&self) -> String {
        format!(
            "host={} user={} password={} dbname={}",
            self.pg_host, self.pg_user, self.pg_password, self.pg_database
        )
    }
}

// getters
impl PostgresDBConfig {
    #[must_use]
    pub fn pg_host(&self) -> &str {
        &self.pg_host
    }

    #[must_use]
    pub fn pg_user(&self) -> &str {
        &self.pg_user
    }

    #[must_use]
    pub fn pg_password(&self) -> &str {
        &self.pg_password
    }

    #[must_use]
    pub fn pg_database(&self) -> &str {
        &self.pg_database
    }

    #[must_use]
    pub const fn pg_port(&self) -> u16 {
        self.pg_port
    }

    #[must_use]
    pub const fn pg_max_connections(&self) -> u32 {
        self.pg_max_connections
    }
}

impl Display for PostgresDBConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "PostgresDBConfig {{\n  port: {},\n  host: {},\n  user: {},\n  database: {}\n max_connections: {}\n}}",
            self.pg_port,
            self.pg_host,
            self.pg_user,
            self.pg_database,
            self.pg_max_connections,
        )
    }
}
