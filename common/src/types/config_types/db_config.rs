use std::fmt::{Debug, Display, Formatter};

const CLIENT_NAME: &str = "rsmgclient/2.0.2";

/// MemGraphConfig represents the configuration for connecting to a Memgraph instance.
///
/// # Fields
///
/// * `port`: The port number to connect to at the server host. The default port is 7687.
/// * `host`: The DNS resolvable name of the host to connect to. Exactly one of `host` and `address`
/// parameters must be specified.
/// * `username`: The username to connect as.
/// * `password`: The password to be used if the server demands password authentication.
/// * `client_name`: The alternate name and version of the client to send to the server. The default
/// is "MemgraphBolt/0.1".
#[derive(Debug, Eq, Clone, PartialEq)]
pub struct DBConfig {
    /// Port number to connect to at the server host. The default port is 7687.
    port: u16,
    /// DNS resolvable name of the host to connect to. Exactly one of `host` and `address` parameters
    /// must be specified.
    host: Option<String>,
    /// Username to connect as.
    username: Option<String>,
    /// Password to be used if the server demands password authentication.
    password: Option<String>,
    /// Alternate name and version of the client to send to server. The default is "MemgraphBolt/0.1".
    client_name: String,
}

impl DBConfig {
    /// Creates a new MemGraphConfig for a connection without authentication.
    pub fn new_connection(port: u16, host: Option<String>) -> Self {
        Self {
            port,
            host,
            ..Default::default()
        }
    }

    /// Creates a new MemGraphConfig for authentication.
    pub fn new_authentication(username: Option<String>, password: Option<String>) -> Self {
        Self {
            username,
            password,
            ..Default::default()
        }
    }

    /// Creates a new MemGraphConfig for a connection with authentication.
    pub fn new_connection_with_authentication(
        port: u16,
        host: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            port,
            host,
            username,
            password,
            ..Default::default()
        }
    }
}

// getters
impl DBConfig {
    /// Returns the port number to connect to at the server host.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the DNS resolvable name of the host to connect to.
    pub fn host(&self) -> &Option<String> {
        &self.host
    }

    /// Returns the username to connect as.
    pub fn username(&self) -> &Option<String> {
        &self.username
    }

    /// Returns the password to be used if the server demands password authentication.
    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    /// Returns the alternate name and version of the client to send to server.
    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

impl Default for DBConfig {
    /// Returns the default MemGraphConfig.
    fn default() -> Self {
        Self {
            port: 7687,
            host: None,
            username: None,
            password: None,
            client_name: String::from(CLIENT_NAME),
        }
    }
}

impl Display for DBConfig {
    /// Formats the MemGraphConfig as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "DBConfig {{ port: {}, host: {:?}, username: {:?}, password: {:?}, client_name: {:?} }}",
               self.port, self.host, self.username, self.password, self.client_name
        )
    }
}
