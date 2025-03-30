pub(crate) mod alpha_stream;
mod binary_protocol;

// Constants used in both, AlphaSocket and AlphaListener
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 500;
// 500 milliseconds default timeout
pub(crate) const DEFAULT_BUFFER_SIZE: usize = 64 * 1024;
