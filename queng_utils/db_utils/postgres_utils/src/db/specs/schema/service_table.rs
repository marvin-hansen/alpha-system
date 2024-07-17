use crate::db::{Specs, SERVICE_TABLE};

impl Specs {
    pub(crate) fn generate_service_table_ddl(&self) -> String {
        format!(
            "CREATE TABLE IF NOT EXISTS public.{SERVICE_TABLE} (
            id               smallint PRIMARY KEY,
            name             VARCHAR NOT NULL,
            version          smallint NOT NULL,
            online           boolean NOT NULL,
            description      VARCHAR NOT NULL,
            health_check_uri VARCHAR NOT NULL,
            base_uri        VARCHAR NOT NULL,
            dependencies    smallint[],
            exposure         smallint NOT NULL
        );"
        )
    }
}
