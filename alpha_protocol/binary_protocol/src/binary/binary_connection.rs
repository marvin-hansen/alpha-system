use std::path::Path;
use stream_errors::ClientError;

pub trait ConnectionClient {
    /// Connect to the server.
    /// If the client is already connected, do nothing.
    fn connect<P: AsRef<Path>>(&self, path: P) -> Result<(), ClientError>;

    /// Shutdown the client and release all the resources.
    /// Ensure to call this when you're done with the client
    /// to release all resources and close open connections.
    fn shutdown(&self) -> Result<(), ClientError>;
}
