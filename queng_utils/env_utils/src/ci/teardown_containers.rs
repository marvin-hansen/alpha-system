use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    // teardown CI instance of test environment
    pub async fn teardown_containers(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("Get docker util");
        let mut docker_util = self
            .get_docker_util()
            .expect("[TestEnv:CI]: Failed to get docker util");

        self.dbg_print("Get clickhouse container id");
        let container_id = self.clickhouse_container_name();

        self.dbg_print("Remove clickhouse container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        self.dbg_print("Get api proxy container id");
        let container_id = self.api_proxy_container_name();
        self.dbg_print("Remove api proxy container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown api proxy container");

        Ok(())
    }
}
