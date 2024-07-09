use crate::prelude::{EnvUtil, EnvironmentError};

impl EnvUtil {
    pub async fn teardown_db(&self) -> Result<(), EnvironmentError> {
        self.dbg_print("[teardown_ci]: teardown clickhouse container");
        return match self.teardown_ci_clickhouse().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
    }

    pub async fn teardown_ci(&self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[teardown_ci]: teardown clickhouse container");
        match self.teardown_ci_clickhouse().await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        self.dbg_print("[teardown_ci]: teardown api proxy container");
        match self.teardown_ci_api_proxy().await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub async fn teardown_ci_clickhouse(&self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[teardown_ci_clickhouse]: Get clickhouse util");
        let ch_util = self.clickhouse_util().await.expect("Failed to get CH util");

        ch_util
            .teardown_all_db()
            .await
            .expect("Failed to teardown DB");

        Ok(())
    }

    pub async fn teardown_ci_api_proxy(&self) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[teardown_ci_api_proxy]: Get docker util");
        let docker_util = self.docker_util();

        self.dbg_print("[teardown_ci_api_proxy]: Get container id");
        let container_id = self.api_proxy_container_name();

        self.dbg_print("[teardown_ci_api_proxy]: Stop and remove container");
        docker_util
            .stop_container(container_id)
            .expect("[TestEnv:CI/teardown_ci_api_proxy]: Failed to teardown api_proxy container");

        Ok(())
    }
}
