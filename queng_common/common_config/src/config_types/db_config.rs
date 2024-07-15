use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

const BUFFER_SIZE: usize = 50_000;

/// Configuration for the Postgres database.
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBConfig {
    /// ILP ort number to connect to at the server host. The default port for the line protocol is 9009.
    port: u16,
    /// DNS resolvable name of the host to connect to.
    host: String,
    /// ILP Buffer size before flushing to the server.
    buffer_size: usize,
    // Postgres authentication parameters.
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

impl DBConfig {
    pub fn new(port: u16, host: String) -> Self {
        Self {
            port,
            host,
            buffer_size: BUFFER_SIZE,
            // Default Postgres authentication parameters.
            // https://questdb.io/docs/develop/connect/
            pg_user: "admin".to_string(),
            pg_password: "quest".to_string(),
            pg_database: "qdb".to_string(),
            pg_port: 8812,
            pg_max_connections: 10,
        }
    }

    pub fn new_with_pg_config(
        port: u16,
        host: String,
        pg_user: String,
        pg_password: String,
        pg_database: String,
        pg_port: u16,
        pg_max_connections: u32,
    ) -> Self {
        Self {
            port,
            host,
            buffer_size: BUFFER_SIZE,
            pg_user,
            pg_password,
            pg_database,
            pg_port,
            pg_max_connections,
        }
    }
}

impl DBConfig {
    /// Generates a PostgreSQL connection string from the DBConfig.
    ///
    /// The connection string contains the parameters required to connect to
    /// QuestDB's PostgreSQL endpoint, including:
    ///
    /// - user
    /// - password
    /// - host
    /// - port
    pub fn pg_connection_string(&self) -> String {
        // https://questdb.io/docs/develop/query-data/#postgresql-wire-protocol
        format!(
            "user={} password={} host={} port={} dbname={}",
            self.pg_user, self.pg_password, self.host, self.pg_port, self.pg_database,
        )
    }

    pub fn pg_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_user, self.pg_password, self.host, self.pg_port, self.pg_database,
        )
    }
}

// getters
impl DBConfig {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn pg_user(&self) -> &str {
        &self.pg_user
    }

    pub fn pg_password(&self) -> &str {
        &self.pg_password
    }

    pub fn pg_database(&self) -> &str {
        &self.pg_database
    }

    pub fn pg_port(&self) -> u16 {
        self.pg_port
    }

    pub fn pg_max_connections(&self) -> u32 {
        self.pg_max_connections
    }
}

impl Display for DBConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "DBConfig {{\n  port: {},\n  host: {},\n  pg_user: {},\n  pg_database: {}\n pg_port: {}\n}}",
            self.port,
            self.host,
            self.pg_user,
            self.pg_database,
            self.pg_port,
        )
    }
}
