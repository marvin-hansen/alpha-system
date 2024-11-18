use crate::PostgresSMDBManager;
use common_config::{Endpoint, ServiceConfig, ServiceID};
use common_errors::PostgresDBError;
use pg_smdb::service;

impl PostgresSMDBManager {
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
    ///
    pub async fn insert_service(
        &self,
        service_config: &ServiceConfig,
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_service");
        let conn = &mut self.get_connection();

        match service::Service::create(conn, service_config) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Imports a collection of service configurations into the database.
    ///
    /// # Arguments
    ///
    /// * `services` - A slice of `ServiceConfig` objects.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if the insertion operation fails.
    ///
    pub async fn insert_service_collection(
        &self,
        services: &[ServiceConfig],
    ) -> Result<(), PostgresDBError> {
        self.dbg_print("insert_service_collection");
        let conn = &mut self.get_connection();

        match service::Service::insert_service_collection(conn, services) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    /// Returns the number of services in the database.
    ///
    /// # Returns
    ///
    /// * `Result<u64, PostgresDBError>` - A result indicating success or failure.
    ///    The number of services is returned as a `u64` if successful.
    ///
    pub async fn count_services(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_services");
        let conn = &mut self.get_connection();

        match service::Service::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
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
    ///    If the service exists, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
    pub async fn check_if_service_id_exists(
        &self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_exists");
        let conn = &mut self.get_connection();

        match service::Service::check_if_service_id_exists(conn, *id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    /// Checks if all services with the collection of given service
    /// IDs exist in the database.
    ///
    /// # Arguments
    ///
    /// * `services` - The IDs of the services to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    ///    If all services exist, returns `Ok(true)`, otherwise `Ok(false)`.
    ///
    pub async fn check_if_services_exists(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_services_exists");

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

    /// Checks if a service with the given ID is online.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to check.
    ///
    /// # Returns
    ///
    /// * `Result<bool, PostgresDBError>` - A result indicating success or failure.
    ///    If the service exists and is online, returns `Ok(true)`.
    ///    If the service does not exist, returns `Ok(false)`.
    ///
    pub async fn check_if_service_id_online(
        &self,
        id: &ServiceID,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_service_id_online");
        let conn = &mut self.get_connection();

        match service::Service::check_if_service_id_online(conn, *id) {
            Ok(online) => Ok(online),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
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
    ///    If all services exist and are online, returns `Ok(true)`.
    ///    If any service does not exist or is not online, returns `Ok(false)`.
    ///
    pub async fn check_if_services_online(
        &self,
        services: &Vec<ServiceID>,
    ) -> Result<bool, PostgresDBError> {
        for id in services {
            match self.check_if_service_id_online(id).await {
                Ok(online) => {
                    if !online {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(true)
    }
    /// Retrieves all online services from the database.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a vector of all online services in the database.
    ///    If the operation fails, returns a `PostgresDBError`.
    ///
    pub async fn get_all_online_services(&self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        self.dbg_print("get_all_online_services");
        let conn = &mut self.get_connection();

        match service::Service::get_all_online_services(conn) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all offline services from the database.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a vector of all offline services in the database.
    ///    If the operation fails, returns a `PostgresDBError`.
    ///
    pub async fn get_all_offline_services(&self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        self.dbg_print("get_all_offline_services");
        let conn = &mut self.get_connection();

        match service::Service::get_all_offline_services(conn) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all service dependencies for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to retrieve dependencies for
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ServiceID>, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a vector of all service dependencies for the given service ID.
    ///    If the operation fails, returns a `PostgresDBError`.
    ///
    pub async fn get_all_service_dependencies(
        &self,
        id: &ServiceID,
    ) -> Result<Vec<ServiceID>, PostgresDBError> {
        self.dbg_print("get_all_service_dependencies");
        let conn = &mut self.get_connection();

        match service::Service::get_all_service_dependencies(conn, *id) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Retrieves all service endpoints for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to retrieve endpoints for
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Endpoint>, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a vector of all service endpoints for the given service ID.
    ///    If the operation fails, returns a `PostgresDBError`.
    ///
    pub async fn get_all_service_endpoints(
        &self,
        id: &ServiceID,
    ) -> Result<Vec<Endpoint>, PostgresDBError> {
        self.dbg_print("get_all_service_endpoints");
        let conn = &mut self.get_connection();

        match service::Service::get_all_service_endpoints(conn, *id) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Sets the service with the given ID to online in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set online.
    ///
    /// # Returns
    ///
    /// * `Result<(), PostgresDBError>` - A result indicating success or failure.
    ///    If the service was set to online, returns `Ok(())`.
    ///
    pub async fn set_service_online(&self, id: &ServiceID) -> Result<(), PostgresDBError> {
        self.dbg_print("set_service_online");
        let conn = &mut self.get_connection();

        match service::Service::set_service_online(conn, *id) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::SetFieldFailed(e.to_string())),
        }
    }

    /// Sets the service with the given ID to offline in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the service to set offline.
    ///
    /// # Returns
    ///
    /// * `Result<(), PostgresDBError>` - A result indicating success or failure.
    ///    If the service was set to offline, returns `Ok(())`.
    ///
    pub async fn set_service_offline(&self, id: &ServiceID) -> Result<(), PostgresDBError> {
        self.dbg_print("set_service_offline");
        let conn = &mut self.get_connection();

        match service::Service::set_service_offline(conn, *id) {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresDBError::SetFieldFailed(e.to_string())),
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
    ///    If successful, returns a `Some(ServiceConfig)` with the service data.
    ///    If the service does not exist, returns `Ok(None)`.
    ///
    pub async fn read_service_by_id(
        &self,
        id: &ServiceID,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_service_by_id");
        let conn = &mut self.get_connection();

        match service::Service::check_if_service_id_exists(conn, *id) {
            Ok(exists) => {
                if !exists {
                    return Ok(None);
                }
            }
            Err(e) => return Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        };

        match service::Service::read(conn, *id) {
            Ok(svc) => Ok(Some(svc)),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    /// Reads all services from the database.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ServiceConfig>, PostgresDBError>` - A result indicating success or failure.
    ///    If successful, returns a vector of all services in the database.
    ///
    /// # Errors
    ///
    /// Returns an PostgresDBError if the query fails.
    ///
    pub async fn read_all_services(&self) -> Result<Vec<ServiceConfig>, PostgresDBError> {
        self.dbg_print("read_all_services");
        let conn = &mut self.get_connection();

        match service::Service::read_all(conn) {
            Ok(services) => Ok(services),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
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
    ///    If successful, returns the updated service data.
    ///    If the service does not exist, returns `Ok(None)`.
    ///
    /// # Errors
    ///
    /// Returns a PostgresDBError if the update fails.
    ///
    pub async fn update_service(
        &self,
        data: ServiceConfig,
    ) -> Result<Option<ServiceConfig>, PostgresDBError> {
        self.dbg_print("update_service");
        let conn = &mut self.get_connection();

        let id = data.svc_id().to_owned();
        match service::Service::check_if_service_id_exists(conn, id) {
            Ok(exists) => {
                if !exists {
                    return Ok(None);
                }
            }
            Err(e) => return Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        };

        match service::Service::update(conn, data.svc_id(), &data) {
            Ok(svc) => Ok(Some(svc)),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
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
    ///    If the service was deleted, returns `Ok(true)`.
    ///    If the service does not exist, returns `Ok(false)`.
    ///
    pub async fn delete_service(&self, id: &ServiceID) -> Result<bool, PostgresDBError> {
        self.dbg_print("delete_service");
        let conn = &mut self.get_connection();

        match service::Service::check_if_service_id_exists(conn, *id) {
            Ok(exists) => {
                if !exists {
                    return Ok(false);
                }
            }
            Err(e) => return Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        };

        match service::Service::delete(conn, *id) {
            Ok(_) => Ok(true),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
