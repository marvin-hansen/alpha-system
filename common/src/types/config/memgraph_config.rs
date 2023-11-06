use std::fmt::{Debug, Display, Formatter};

use rsmgclient::{ConnectParams, SSLMode};

const CLIENT_NAME: &str = "rsmgclient/2.0.2";

#[derive(Debug, Eq, PartialEq)]
pub struct MemGraphConfig {
    /// Port number to connect to at the server host. Default port is 7687.
    port: u16,
    /// DNS resolvable name of host to connect to. Exactly one of host and address parameters must
    /// be specified.
    host: Option<String>,
    /// Username to connect as.
    username: Option<String>,
    /// Password to be used if the server demands password authentication.
    password: Option<String>,
    /// Alternate name and version of the client to send to server. Default is
    /// "MemgraphBolt/0.1".
    client_name: String,
}

impl MemGraphConfig {
    pub fn new_connection(port: u16, host: Option<String>) -> Self {
        Self {
            port,
            host,
            ..Default::default()
        }
    }

    pub fn new_authentication(username: Option<String>, password: Option<String>) -> Self {
        Self {
            username,
            password,
            ..Default::default()
        }
    }

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

    pub fn get_connect_params(&self) -> ConnectParams {
        ConnectParams {
            host: self.host.clone(),
            port: self.port,
            sslmode: SSLMode::Disable,
            username: self.username.clone(),
            password: self.password.clone(),
            client_name: self.client_name.clone(),
            ..Default::default()
        }
    }
}

// getters
impl MemGraphConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn host(&self) -> &Option<String> {
        &self.host
    }
    pub fn username(&self) -> &Option<String> {
        &self.username
    }
    pub fn password(&self) -> &Option<String> {
        &self.password
    }
    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

impl Default for MemGraphConfig {
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

impl Display for MemGraphConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
