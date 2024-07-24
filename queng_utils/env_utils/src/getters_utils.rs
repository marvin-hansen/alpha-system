use clickhouse_utils::error::ClickHouseUtilError;
use clickhouse_utils::ClickhouseUtil;
use postgres_utils::prelude::PostgresUtilError;
use postgres_utils::PostgresUtil;
use specs_utils::db_specs;
use specs_utils::prelude::clickhouse_container_specs;

use crate::EnvUtil;

impl EnvUtil {
    pub(crate) async fn get_new_clickhouse_util(
        &self,
    ) -> Result<ClickhouseUtil, ClickHouseUtilError> {
        let container_config = clickhouse_container_specs();

        // DB connection string
        let dsn = &format!(
            "{}:{}",
            container_config.url(),
            container_config.connection_port(),
        );

        if self.dbg {
            ClickhouseUtil::with_debug(dsn).await
        } else {
            ClickhouseUtil::new(dsn).await
        }
    }

    pub(crate) async fn get_new_postgres_util(&self) -> Result<PostgresUtil, PostgresUtilError> {
        let config = db_specs::get_postgres_config(&self.env);

        let tsn = config.tsn();

        if self.dbg {
            PostgresUtil::with_debug(&tsn).await
        } else {
            PostgresUtil::new(&tsn).await
        }
    }
}
