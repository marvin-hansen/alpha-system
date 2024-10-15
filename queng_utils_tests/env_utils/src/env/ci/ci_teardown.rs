use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    pub async fn teardown_ci(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("[teardown_ci]: teardown postgres");
        match self.teardown_postgres().await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        self.dbg_print("[teardown_ci]: teardown api proxy");
        match self.teardown_api_proxy().await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
