use crate::error::SpecDBError;

mod error;

pub struct SpecDBManager {}

impl SpecDBManager {
    pub async fn new() -> Result<Self, SpecDBError> {
        Ok(Self {})
    }
}
