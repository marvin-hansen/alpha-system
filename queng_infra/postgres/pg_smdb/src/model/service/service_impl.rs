use crate::model::endpoint_type::Endpoint;
use crate::model::service::{CreateService, Service, UpdateService};
use crate::schema::smdb::service::dsl::{dependencies, endpoints, online, service, service_id};
use crate::Connection;
use common_config::Endpoint as CommonEndpoint;
use common_config::ServiceConfig as CommonServiceConfig;
use common_config::ServiceID as CommonServiceID;
use diesel::{
    insert_into, ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};

impl Service {
    /// Creates a new service in the database with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `svc` - A reference to a `CommonServiceConfig` containing the configuration for the new service
    ///
    /// # Returns
    ///
    /// * `QueryResult<ServiceConfig>` - The newly created service configuration
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Unique constraint violations (if service_id already exists)
    /// * Invalid data in service configuration (constraint violations)
    /// * Serialization errors when converting between types
    ///
    pub fn create(
        db: &mut Connection,
        svc: &CommonServiceConfig,
    ) -> QueryResult<CommonServiceConfig> {
        let item = CreateService::from_common_svc_config(svc);
        insert_into(crate::schema::smdb::service::table)
            .values(item)
            .get_result::<Self>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Inserts multiple services into the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `svc` - A slice of `CommonServiceConfig` containing the configurations for the services to add
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if all services were inserted successfully
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Unique constraint violations
    /// * Invalid data in any service configuration
    /// * Batch insertion failures
    /// * Transaction rollback if any service fails to insert
    ///
    pub fn insert_service_collection(
        db: &mut Connection,
        svc: &[CommonServiceConfig],
    ) -> QueryResult<bool> {
        let items = svc
            .iter()
            .map(CreateService::from_common_svc_config)
            .collect::<Vec<CreateService>>();

        match insert_into(crate::schema::smdb::service::table)
            .values(&items)
            .execute(db)
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the number of services in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// * `QueryResult<u64>` - Total number of services
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Type conversion errors when converting count from i64 to u64
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        service.count().get_result::<i64>(db).map(|c| c as u64)
    }

    /// Checks if a service ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to check
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if the service exists, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Note: Not finding the service is NOT an error, it returns `Ok(false)`
    ///
    pub fn check_if_service_id_exists(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<bool> {
        service
            .find(param_service_id.as_i32())
            .first::<Self>(db)
            .optional()
            .map(|arg0: Option<Self>| Option::is_some(&arg0))
    }

    /// Checks if a service ID is online.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to check
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if the service is online, `false` if offline or not found
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    ///
    pub fn check_if_service_id_online(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<bool> {
        match service
            .filter(service_id.eq(param_service_id.as_i32()))
            .select(online)
            .first::<bool>(db)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    /// Retrieves all online services from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<ServiceConfig>>` - List of all online services
    ///   Returns an empty vector if no services are online
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors
    /// * Memory allocation errors for large result sets
    ///
    pub fn get_all_online_services(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .filter(online.eq(true))
            .select(Self::as_returning())
            .load::<Self>(db)
            .map(|s| s.iter().map(Self::to_common_svc_config).collect())
    }

    /// Retrieves all offline services from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<ServiceConfig>>` - List of all offline services
    ///   Returns an empty vector if no services are offline
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors
    /// * Memory allocation errors for large result sets
    ///
    pub fn get_all_offline_services(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .filter(online.eq(false))
            .select(Self::as_returning())
            .load::<Self>(db)
            .map(|s| s.iter().map(Self::to_common_svc_config).collect())
    }

    /// Retrieves all service dependencies for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to retrieve dependencies for
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<ServiceID>>` - List of all service dependencies
    ///   Returns an empty vector if the service has no dependencies
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Data deserialization errors
    ///
    pub fn get_all_service_dependencies(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<Vec<CommonServiceID>> {
        service
            .filter(service_id.eq(param_service_id.as_i32()))
            .select(dependencies)
            .get_result::<Vec<Option<i32>>>(db)
            .map(|s| {
                s.iter()
                    .flatten()
                    .map(|s| CommonServiceID::from(*s))
                    .collect()
            })
    }

    /// Retrieves all endpoints for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to retrieve endpoints for
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<Endpoint>>` - List of all service endpoints
    ///   Returns an empty vector if the service has no endpoints
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Data deserialization errors
    ///
    pub fn get_all_service_endpoints(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<Vec<CommonEndpoint>> {
        service
            .filter(service_id.eq(param_service_id.as_i32()))
            .select(endpoints)
            .get_result::<Vec<Option<Endpoint>>>(db)
            .map(|s| {
                s.iter()
                    .flatten()
                    .map(super::super::endpoint_type::Endpoint::to_common_endpoint)
                    .collect()
            })
    }

    /// Retrieves the service configuration for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to retrieve
    ///
    /// # Returns
    ///
    /// * `QueryResult<ServiceConfig>` - The service configuration if found
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Data deserialization errors
    ///
    pub fn read(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<CommonServiceConfig> {
        service
            .filter(service_id.eq(param_service_id.as_i32()))
            .get_result::<Self>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Retrieves all service configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// * `QueryResult<Vec<ServiceConfig>>` - List of all service configurations
    ///   Returns an empty vector if no services exist
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Data deserialization errors
    /// * Memory allocation errors for large result sets
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .load::<Self>(db)
            .map(|s| s.iter().map(Self::to_common_svc_config).collect())
    }

    /// Sets the online status of a service.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service
    /// * `param_online` - Whether to set the service online or offline
    ///
    /// # Returns
    ///
    /// * `QueryResult<()>` - Success if the status was updated
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Concurrent modification conflicts
    ///
    fn set_svc_online(
        db: &mut Connection,
        param_service_id: CommonServiceID,
        param_online: bool,
    ) -> QueryResult<()> {
        match diesel::update(service.filter(service_id.eq(param_service_id.as_i32())))
            .set(online.eq(param_online))
            .execute(db)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sets the online status of the service with the given ID.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to set online or offline
    ///
    /// # Returns
    ///
    /// * `QueryResult<()>` - Success if the status was updated
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Concurrent modification conflicts
    ///
    pub fn set_service_online(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<()> {
        Self::set_svc_online(db, param_service_id, true)
    }

    /// Sets the service with the given ID to offline.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to set offline
    ///
    /// # Returns
    ///
    /// * `QueryResult<()>` - Success if the status was updated
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Concurrent modification conflicts
    ///
    pub fn set_service_offline(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<()> {
        Self::set_svc_online(db, param_service_id, false)
    }

    /// Updates a service in the database with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to update
    /// * `item` - A reference to a `CommonServiceConfig` containing the updated configuration
    ///
    /// # Returns
    ///
    /// * `QueryResult<ServiceConfig>` - The updated service configuration
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Service ID does not exist
    /// * Constraint violations in the updated configuration
    /// * Data validation errors
    /// * Concurrent modification conflicts
    ///
    pub fn update(
        db: &mut Connection,
        param_service_id: &CommonServiceID,
        item: &CommonServiceConfig,
    ) -> QueryResult<CommonServiceConfig> {
        let item = UpdateService::from_common_svc_config(item);
        diesel::update(service.filter(service_id.eq(param_service_id.as_i32())))
            .set(item)
            .returning(Self::as_returning())
            .get_result::<Self>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Deletes a service from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to a postgres database connection
    /// * `param_service_id` - The ID of the service to delete
    ///
    /// # Returns
    ///
    /// * `QueryResult<usize>` - Number of rows affected:
    ///   - Returns `Ok(0)` if the service didn't exist
    ///   - Returns `Ok(1)` if the service was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Foreign key constraint violations (if service is referenced elsewhere)
    /// * Concurrent modification conflicts
    ///
    pub fn delete(db: &mut Connection, param_service_id: CommonServiceID) -> QueryResult<usize> {
        diesel::delete(service.filter(service_id.eq(param_service_id.as_i32()))).execute(db)
    }
}
