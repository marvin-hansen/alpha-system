mod handler;
mod types;

pub use types::error::ImsDataClientError;

pub struct ImsDataClient {}

impl ImsDataClient {
    pub async fn new(_client_id: u16) -> Result<Self, ImsDataClientError> {
        Ok(Self {})
    }
}
