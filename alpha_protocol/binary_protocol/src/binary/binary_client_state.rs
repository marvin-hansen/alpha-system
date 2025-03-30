/// Connection state of the client
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ClientState {
    /// The client is authenticating.
    Authenticating = 0,
    /// The client is connected and authenticated.
    Authenticated = 1,
    /// The client is connecting.
    Connecting = 2,
    /// The client is connected.
    Connected = 3,
    /// The client is shutting down.
    ShuttingDown = 4,
    /// The client is shutdown.
    Shutdown = 5,
    /// The client is disconnecting.
    Disconnecting = 6,
    /// The client is shutdown and disconnected.
    #[default]
    Disconnected = 7,
}

impl From<ClientState> for u8 {
    #[inline]
    fn from(value: ClientState) -> Self {
        value as u8
    }
}

impl From<u8> for ClientState {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => ClientState::Authenticating,
            1 => ClientState::Authenticated,
            2 => ClientState::Connecting,
            3 => ClientState::Connected,
            4 => ClientState::ShuttingDown,
            5 => ClientState::Shutdown,
            6 => ClientState::Disconnecting,
            7 => ClientState::Disconnected,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
