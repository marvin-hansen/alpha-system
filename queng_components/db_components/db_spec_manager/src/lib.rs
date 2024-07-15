use error::spec_db_error::SpecDBError;

mod db_portfolio;
mod db_service;
mod error;
pub mod prelude;
mod types;

pub struct SpecDBManager {}

impl SpecDBManager {
    pub async fn new() -> Result<Self, SpecDBError> {
        Ok(Self {})
    }
}
