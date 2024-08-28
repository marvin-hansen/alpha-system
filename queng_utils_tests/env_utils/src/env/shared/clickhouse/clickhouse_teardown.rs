use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    pub(crate) async fn teardown_clickhouse(&self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[teardown_ci_clickhouse]: Get clickhouse util");
        let ch_util = &self
            .get_new_clickhouse_util()
            .await
            .expect("Failed to get ClickhouseUtil");

        ch_util
            .teardown_all_db(true)
            .await
            .expect("Failed to teardown DB");

        Ok(())
    }
}
