use postgres_utils::PostgresUtil;

use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    pub(crate) async fn _import_all_data(
        &self,
        _pg_utils: &PostgresUtil,
    ) -> Result<(), EnvironmentError> {
        Ok(())
    }
}
