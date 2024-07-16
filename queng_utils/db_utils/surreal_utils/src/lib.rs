mod db;
mod errors;
pub mod prelude;
mod query_utils;
mod types;

use crate::db::Specs;
use crate::prelude::SurrealUtilError;

pub struct SurrealUtil {
    pub specs: Specs,
}

impl SurrealUtil {
    pub async fn new() -> Result<Self, SurrealUtilError> {
        Self::build(false).await
    }

    pub async fn with_debug() -> Result<Self, SurrealUtilError> {
        Self::build(true).await
    }

    async fn build(dbg: bool) -> Result<Self, SurrealUtilError> {
        if dbg {
            println!("[ClickhouseUtil]: Debug mode enabled");
        }

        let specs = Specs::new(dbg);

        Ok(Self { specs })
    }
}
