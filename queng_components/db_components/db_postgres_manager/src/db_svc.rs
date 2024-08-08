use common_config::prelude::{ServiceConfig, ServiceID};
use common_pg_queries::{service_insert, service_query, service_update};

use crate::error::PostgresDBError;
use crate::PostgresDBManager;

const SYSTEM_SCHEMA: &str = "system";

const SERVICE_TABLE: &str = "service";

impl PostgresDBManager {
    /// Inserts a new service into the database.
    ///
    /// # Arguments
    ///
    /// * `data` - The service configuration to insert.
    ///
    /// # Returns
    ///
    /// * `Result<(), PostgresDBError>` - A result indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the insert fails.
    pub async fn insert_service(&self, data: &ServiceConfig) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_service");

        let query = service_insert::build_insert_service_query(data);
        match self.execute_insert_query(&query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Returns the number of services in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    /// The number of services is returned as a `u64` if successful.
    pub async fn count_services(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_services");

        match self.execute_count_query(SYSTEM_SCHEMA, SERVICE_TABLE).await {
            Ok(count) => Ok(count),
            Err(e) => Err(e),
        }
    }

    /// Checks if a service with the given ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the service exists, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
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

    /// Checks if services with the given IDs exist in the database.
    ///
    /// # Arguments
    ///
    /// * `services` - The IDs of the services to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If all services exist, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
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

    /// Checks if a service with the given ID is online in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the service exists and is online, returns `Ok(true)`.
    /// If the service does not exist, returns `Ok(false)`.
    ///
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

    /// Checks if services with the given IDs are online in the database.
    ///
    /// # Arguments
    ///
    /// * `services` - The IDs of the services to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If all services exist and are online, returns `Ok(true)`.
    /// If any service does not exist or is not online, returns `Ok(false)`.
    ///
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

    /// Sets the service with the given ID to online in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set online.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the service was set to online, returns `Ok(true)`.
    /// If the service does not exist, returns `Ok(false)`.
    ///
    pub async fn set_service_online(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("set_service_online");
        self.set_svc_online(id, true).await
    }

    /// Sets the service with the given ID to offline in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set offline.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the service was set to offline, returns `Ok(true)`.
    /// If the service does not exist, returns `Ok(false)`.
    ///
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

    /// Reads a service from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to read.
    ///
    /// # Returns
    ///
    /// * `Result<Option<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    /// If successful, returns a `Some(ServiceConfig)` with the service data.
    /// If the service does not exist, returns `Ok(None)`.
    ///
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

    /// Reads all services from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    /// If successful, returns a vector of all services in the database.
    ///
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

    /// Updates a service in the database.
    ///
    /// # Arguments
    ///
    /// * `data` - The updated service data.
    ///
    /// # Returns
    ///
    /// * `Result<Option<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    /// If successful, returns the updated service data.
    /// If the service does not exist, returns `Ok(None)`.
    ///
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

    /// Deletes a service from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to delete.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    /// If the service was deleted, returns `Ok(true)`.
    /// If the service does not exist, returns `Ok(false)`.
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
