use crate::db::specs::{Specs, DB_NAME, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    //CREATE TABLE IF NOT EXISTS specs.services
    // (
    //     svc_id UInt8,
    //     svc_name String,
    //     svc_version UInt8,
    //     online Boolean,
    //     description String,
    //     health_check_uri String,
    //     base_uri String,
    //     dependencies Array(UInt8),
    //     exposure UInt8,
    //     endpoint Tuple (endpoint_name String, endpoint_version UInt8, description String, uri String, port UInt16, protocol UInt8, encoding UInt8),
    //     metrics Tuple (uri String, host String, port UInt16),
    // )
    //     ENGINE = MergeTree()
    //     PRIMARY KEY (svc_id)
    //     ;

    pub(crate) async fn create_service_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_service_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_service_table_ddl(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS {DB_NAME}.{SERVICES_TABLE}\
        (
            svc_id UInt8,
            name String,
            version UInt8,
            online Boolean,
            description String,
            health_check_uri String,
            base_uri String,
            dependencies Array(UInt8),
            exposure UInt8,
            endpoint Nested (name String, version UInt8, description String, uri String, port UInt16, protocol UInt8, encoding UInt8),
            metrics Nested (uri String, host String, port UInt16),
    )
    ENGINE = MergeTree()
    PRIMARY KEY (svc_id)
    SETTINGS index_granularity = 5
    ;
    "
        )
    }
}
