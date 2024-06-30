use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    // teardown CI instance of test environment
    pub async fn teardown_ci(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("Get clickhouse utils");
        let ch_utils = self.clickhouse_util().await.expect("");
        self.dbg_print("Get docker util");
        let mut docker_util = self.docker_util();

        self.dbg_print("Remove all meta data tables");
        ch_utils
            .drop_metadata_tables()
            .await
            .expect("Failed to drop all meta data tables");

        self.dbg_print("Remove all databases");
        ch_utils
            .teardown_db()
            .await
            .expect("Failed to drop all databases");

        self.dbg_print("Get container id");
        let container_id = self.clickhouse_container_name();

        self.dbg_print("Stop and remove container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        Ok(())
    }
}
