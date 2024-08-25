use common_config::prelude::{ServiceConfig, ServiceID};
use common_pg_queries::{service_insert, service_query, service_update};

use crate::error::PostgresDBError;
use crate::PostgresDBManager;

const SYSTEM_SCHEMA: &str = "system";

const SERVICE_TABLE: &str = "service";

impl PostgresDBManager {
    pub async fn insert_service(&self, data: &ServiceConfig) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_service");

        let query = service_insert::build_insert_service_query(data);
        match self.execute_insert_query(&query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn count_services(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_services");

        match self.execute_count_query(SYSTEM_SCHEMA, SERVICE_TABLE).await {
            Ok(count) => Ok(count),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_service_id_exists(
        &self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_exists");

        let query = service_query::build_check_if_service_id_exists_query(id);
        match self.execute_exists_query(&query).await {
            Ok(exists) => Ok(exists),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_services_exists(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            match self.check_if_service_id_exists(id).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }

    pub async fn check_if_service_id_online(
        &self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_online");

        let query = service_query::build_check_if_service_id_online_query(id);
        match self.execute_exists_query(&query).await {
            Ok(exists) => Ok(exists),
            Err(e) => Err(e),
        }
    }

    pub async fn check_if_services_online(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            match self.check_if_service_id_online(id).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }

    pub async fn set_service_online(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("set_service_online");
        self.set_svc_online(id, true).await
    }

    pub async fn set_service_offline(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("set_service_offline");
        self.set_svc_online(id, false).await
    }

    async fn set_svc_online(&self, id: &ServiceID, online: bool) -> Result<bool, PostgresDBError> {
        let query = service_query::build_set_svc_online_query(id, online);
        match self.execute_exists_query(&query).await {
            Ok(online) => Ok(online),
            Err(e) => Err(e),
        }
    }

    pub async fn read_service_by_id(
        &self,
        id: &ServiceID,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_service_by_id");

        let query = service_query::build_read_service_by_id_query(id);
        match self.client.query_one(&query, &[]).await {
            Ok(row) => {
                let svc = ServiceConfig::from_sql_row(&row);
                Ok(Some(svc))
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::QueryFailed(e.to_string()))
            }
        }
    }

    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_all_services");

        let query = service_query::build_read_all_services_query();
        match self.client.query(&query, &[]).await {
            Ok(rows) => {
                let mut services = Vec::new();
                for row in rows {
                    let svc = ServiceConfig::from_sql_row(&row);
                    services.push(svc);
                }
                Ok(services)
            }
            Err(e) => {
                self.dbg_print(&format!("Query failed: \n {}", query));
                Err(PostgresDBError::QueryFailed(e.to_string()))
            }
        }
    }

    pub async fn update_service(
        &self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("update_service");

        let query = service_update::build_update_service_query(&data);
        match self.execute_query(&query).await {
            Ok(_) => Ok(Some(data)),
            Err(e) => Err(e),
        }
    }

    ///
    pub async fn delete_service(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("delete_service");

        match self.check_if_service_id_exists(id).await {
            Ok(exists) => {
                if !exists {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        };

        let query = service_query::build_delete_service_query(id);
        match self.execute_query(&query).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
