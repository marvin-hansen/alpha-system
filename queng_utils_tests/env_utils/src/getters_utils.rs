use crate::EnvUtil;
use clickhouse_utils::error::ClickHouseUtilError;
use clickhouse_utils::ClickhouseUtil;
use container_specs_clickhouse::clickhouse_container_config;
use db_specs_postgres::prelude::postgres;
use postgres_utils::prelude::PostgresUtilError;
use postgres_utils::PostgresUtil;

impl EnvUtil {
    pub(crate) async fn get_new_clickhouse_util(
        &self,
    ) -> Result<ClickhouseUtil, ClickHouseUtilError> {
        let container_config = clickhouse_container_config();

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
        let config = postgres::get_postgres_config(&self.env);

        let tsn = config.tsn();

        if self.dbg {
            PostgresUtil::with_debug(&tsn).await
        } else {
            PostgresUtil::new(&tsn).await
        }
    }
}
