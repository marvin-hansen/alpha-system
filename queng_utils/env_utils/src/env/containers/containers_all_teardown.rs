use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    // teardown CI instance of test environment
    pub async fn teardown_containers(&self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("Get docker util");
        let docker_util = self.docker_util();

        self.dbg_print("Get clickhouse container id");
        let container_id = self.clickhouse_container_name();

        self.dbg_print("Remove clickhouse container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown clickhouse container");

        self.dbg_print("Get postgres container id");
        let container_id = self.postgres_db_container_name();

        self.dbg_print("Remove postgres container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown postgres container");

        self.dbg_print("Get api proxy container id");
        let container_id = self.api_proxy_container_name();

        self.dbg_print("Remove api proxy container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI]: Failed to teardown api proxy container");

        Ok(())
    }
}
