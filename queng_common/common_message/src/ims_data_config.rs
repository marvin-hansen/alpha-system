use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ImsDataConfig {
    stream_user: String,
    stream_password: String,
    stream_id: String,
    topic_ids: String,
    tcp_server_address: String,
}

impl ImsDataConfig {
    /// Creates a new `ImsDataConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `stream_user` - The username for stream authentication.
    /// * `stream_password` - The password for stream authentication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_ids` - The identifiers of the topics.
    /// * `tcp_server_address` - The tcp server address i.e. "127.0.0.1:8090"
    ///
    /// # Returns
    ///
    /// An `ImsDataConfig` instance .
    ///
    #[must_use]
    pub const fn new(
        stream_user: String,
        stream_password: String,
        stream_id: String,
        topic_ids: String,
        tcp_server_address: String,
    ) -> Self {
        Self {
            stream_user,
            stream_password,
            stream_id,
            topic_ids,
            tcp_server_address,
        }
    }
}

impl ImsDataConfig {
    #[must_use]
    pub fn stream_user(&self) -> &str {
        &self.stream_user
    }

    #[must_use]
    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    #[must_use]
    pub fn topic_ids(&self) -> &str {
        &self.topic_ids
    }

    #[must_use]
    pub fn stream_password(&self) -> &str {
        &self.stream_password
    }

    #[must_use]
    pub fn tcp_server_address(&self) -> &str {
        &self.tcp_server_address
    }
}

impl Display for ImsDataConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
