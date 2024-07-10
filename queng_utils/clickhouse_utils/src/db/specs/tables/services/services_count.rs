use crate::db::specs::{Specs, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub async fn count_services(&self) -> Result<u64, ClickHouseUtilError> {
        return match self.count_rows(SERVICES_TABLE).await {
            Ok(count) => Ok(count),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        };
    }
}
