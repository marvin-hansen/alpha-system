use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    async fn verify_surreal_data_imported(&self) -> Result<bool, EnvironmentError> {
        Ok(false)
    }
}
