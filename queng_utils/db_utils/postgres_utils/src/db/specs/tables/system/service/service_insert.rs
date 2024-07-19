use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub async fn insert_service(&self, _data: &ServiceConfig) -> Result<(), PostgresUtilError> {
        Err(PostgresUtilError::from("Not implemented".to_string()))
    }
}

// insert into service(id,name,version,online,description,health_check_uri,base_uri,dependencies,exposure,endpoint,metrics )
// VALUES(
// 1,
// 'test',
// 1,
// false,
// 'test_description',
// 'test_health_check_uri',
// 'test_base_uri',
// '{1,2,3}',
// 42,
// ROW('test_endpoint', 1, '/', 7070, 3),
// ROW('/metrics', 'localhost', 8080)
// )
// RETURNING *;
