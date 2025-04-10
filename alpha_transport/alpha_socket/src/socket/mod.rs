pub(crate) mod alpha_socket;
pub(crate) mod alpha_socket_with_archiver;

// Constants used in both, AlphaSocket and AlphaListener
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 500;
// 500 milliseconds default timeout
pub(crate) const DEFAULT_BUFFER_SIZE: usize = 64 * 1024;
