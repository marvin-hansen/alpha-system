use rsmgclient::SSLMode;

pub struct MemGraphConfig {
    /// Port number to connect to at the server host. Default port is 7687.
    port: u16,
    /// DNS resolvable name of host to connect to. Exactly one of host and address parameters must
    /// be specified.
    host: Option<String>,
    /// Numeric IP address of host to connect to. This should be in the standard IPv4 address
    /// format. You can also use IPv6 if your machine supports it. Exactly one of host and address
    /// parameters must be specified.
    address: Option<String>,
    /// Username to connect as.
    username: Option<String>,
    /// Password to be used if the server demands password authentication.
    password: Option<String>,
    /// Alternate name and version of the client to send to server. Default is
    /// "MemgraphBolt/0.1".
    client_name: String,
    /// Determines whether a secure SSL TCP/IP connection will be negotiated with the server.
    /// Default value is `SSLMode::Require`.
    sslmode: SSLMode,
    /// This parameter specifies the file name of the client SSL certificate. It is ignored in
    /// case an SSL connection is not made.
    sslcert: Option<String>,
    /// This parameter specifies the location of the secret key used for the client certificate.
    /// This parameter is ignored in case an SSL connection is not made.
    sslkey: Option<String>,
    /// After performing the SSL handshake, `Connection::connect` will call this function providing
    /// the hostname, IP address, public key type and fingerprint and user provided data. If the
    /// function returns a non-zero value, SSL connection will be immediately terminated. This can
    /// be used to implement TOFU (trust on first use) mechanism.
    trust_callback: Option<*const dyn Fn(&String, &String, &String, &String) -> i32>,
    /// Initial value of `lazy` field, defaults to true, Can be changed using `Connection::set_lazy`.
    lazy: bool,
    /// Initial value of `autocommit` field, defaults to false. Can be changed using
    /// `Connection::set_autocommit`.
    autocommit: bool,
}


impl MemGraphConfig {
    pub fn new_unsecure(port: u16, host: Option<String>) -> Self {
        Self { port, host, sslmode: SSLMode::Disable, ..Default::default() }
    }

    pub fn new_with_authentication(port: u16, host: Option<String>, username: Option<String>, password: Option<String>) -> Self {
        Self { port, host, username, password, sslmode: SSLMode::Disable, ..Default::default() }
    }
}

impl Default for MemGraphConfig {
    fn default() -> Self {
        Self {
            port: 7687,
            host: None,
            address: None,
            username: None,
            password: None,
            client_name: String::from("rsmgclient/2.0.2"),
            sslmode: SSLMode::Disable,
            sslcert: None,
            sslkey: None,
            trust_callback: None,
            lazy: true,
            autocommit: false,
        }
    }
}

impl MemGraphConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn host(&self) -> &Option<String> {
        &self.host
    }
    pub fn address(&self) -> &Option<String> {
        &self.address
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
    pub fn sslmode(&self) -> &SSLMode {
        &self.sslmode
    }
    pub fn sslcert(&self) -> &Option<String> {
        &self.sslcert
    }
    pub fn sslkey(&self) -> &Option<String> {
        &self.sslkey
    }
    pub fn trust_callback(&self) -> Option<*const dyn Fn(&String, &String, &String, &String) -> i32> {
        self.trust_callback
    }
    pub fn lazy(&self) -> bool {
        self.lazy
    }
    pub fn autocommit(&self) -> bool {
        self.autocommit
    }
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }
    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }
    pub fn set_password(&mut self, password: Option<String>) {
        self.password = password;
    }
    pub fn set_client_name(&mut self, client_name: String) {
        self.client_name = client_name;
    }
    pub fn set_sslmode(&mut self, sslmode: SSLMode) {
        self.sslmode = sslmode;
    }
    pub fn set_sslcert(&mut self, sslcert: Option<String>) {
        self.sslcert = sslcert;
    }
    pub fn set_sslkey(&mut self, sslkey: Option<String>) {
        self.sslkey = sslkey;
    }
    pub fn set_trust_callback(&mut self, trust_callback: Option<*const dyn Fn(&String, &String, &String, &String) -> i32>) {
        self.trust_callback = trust_callback;
    }
    pub fn set_lazy(&mut self, lazy: bool) {
        self.lazy = lazy;
    }
    pub fn set_autocommit(&mut self, autocommit: bool) {
        self.autocommit = autocommit;
    }
}