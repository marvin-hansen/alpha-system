use crate::model::endpoint_type::Endpoint;
use crate::model::service::{CreateService, Service, UpdateService};
use crate::schema::smdb::service::dsl::*;
use crate::Connection;
use common_config::prelude::Endpoint as CommonEndpoint;
use common_config::prelude::ServiceConfig as CommonServiceConfig;
use common_config::prelude::ServiceID as CommonServiceID;
use diesel::{
    insert_into, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

impl Service {
    /// Creates a new service in the database with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `svc` - a reference to a `CommonServiceConfig` containing the configuration for the new service
    ///
    /// # Returns
    ///
    /// A `QueryResult<ServiceConfig>` containing the configuration of the newly created service,
    /// or an error if the operation fails.
    ///
    pub fn create(
        db: &mut Connection,
        svc: &CommonServiceConfig,
    ) -> QueryResult<CommonServiceConfig> {
        let item = CreateService::from_common_svc_config(svc);
        insert_into(crate::schema::smdb::service::table)
            .values(item)
            .get_result::<Service>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Retrieves the number of services in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<u64>` containing the number of services,
    /// or an error if the operation fails.
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        service.count().get_result::<i64>(db).map(|c| c as u64)
    }

    /// Checks if a service ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to check
    ///
    /// # Returns
    ///
    /// A `QueryResult<bool>` indicating whether the service ID exists or not.
    /// If the operation fails, returns an error.
    ///
    pub fn check_if_service_id_exists(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<bool> {
        match service.find(param_service_id.as_i32()).first::<Service>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Checks if a service ID is online.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to check
    ///
    /// # Returns
    ///
    /// A `QueryResult<bool>` indicating whether the service is online or not.
    /// If the operation fails, returns an error.
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
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<ServiceConfig>>` containing all online services,
    /// or an error if the operation fails.
    ///
    pub fn get_all_online_services(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .filter(online.eq(true))
            .select(Service::as_returning())
            .load::<Service>(db)
            .map(|s| s.iter().map(|s| s.to_common_svc_config()).collect())
    }

    /// Retrieves all offline services from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<ServiceConfig>>` containing all offline services,
    /// or an error if the operation fails.
    ///
    pub fn get_all_offline_services(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .filter(online.eq(false))
            .select(Service::as_returning())
            .load::<Service>(db)
            .map(|s| s.iter().map(|s| s.to_common_svc_config()).collect())
    }

    /// Retrieves all service dependencies for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to retrieve dependencies for
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<ServiceID>>` containing all service dependencies,
    /// or an error if the operation fails.
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
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to retrieve endpoints for
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<Endpoint>>` containing all endpoints for the service,
    /// or an error if the operation fails.
    ///
    pub fn get_all_service_endpoints(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<Vec<CommonEndpoint>> {
        service
            .filter(service_id.eq(param_service_id.as_i32()))
            .select(endpoints)
            .get_result::<Vec<Option<Endpoint>>>(db)
            .map(|s| s.iter().flatten().map(|s| s.to_common_endpoint()).collect())
    }

    /// Retrieves the service configuration for a given service ID from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to retrieve
    ///
    /// # Returns
    ///
    /// A `QueryResult<ServiceConfig>` containing the service configuration,
    /// or an error if the operation fails.
    ///
    pub fn read(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<CommonServiceConfig> {
        service
            .filter(service_id.eq(param_service_id.as_i32()))
            .get_result::<Service>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Retrieves all service configurations from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<Vec<ServiceConfig>>` containing all service configurations,
    /// or an error if the operation fails.
    ///
    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<CommonServiceConfig>> {
        service
            .load::<Service>(db)
            .map(|s| s.iter().map(|s| s.to_common_svc_config()).collect())
    }

    /// Sets the online status of the service with the given ID.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to set online or offline
    ///
    /// # Returns
    ///
    /// A `QueryResult<()>` indicating the success of the operation.
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
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to set offline
    ///
    /// # Returns
    ///
    /// A `QueryResult<()>` indicating the success of the operation.
    ///
    pub fn set_service_offline(
        db: &mut Connection,
        param_service_id: CommonServiceID,
    ) -> QueryResult<()> {
        Self::set_svc_online(db, param_service_id, false)
    }

    /// Sets the online status of a service.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service
    /// * `param_online` - whether to set the service online or offline
    ///
    /// # Returns
    ///
    /// A `QueryResult<()>` indicating the success of the operation.
    ///
    fn set_svc_online(
        db: &mut Connection,
        param_service_id: CommonServiceID,
        param_online: bool,
    ) -> QueryResult<()> {
        match diesel::update(service.filter(service_id.eq(param_service_id.as_i32())))
            .set(online.eq(param_online))
            .returning(Service::as_returning())
            .get_result::<Service>(db)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Updates a service in the database with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to update
    /// * `item` - a reference to a `UpdateService` containing the updated configuration
    ///
    /// # Returns
    ///
    /// A `QueryResult<ServiceConfig>` containing the updated configuration of the service,
    /// or an error if the operation fails.
    ///
    pub fn update(
        db: &mut Connection,
        param_service_id: CommonServiceID,
        item: &UpdateService,
    ) -> QueryResult<CommonServiceConfig> {
        diesel::update(service.filter(service_id.eq(param_service_id.as_i32())))
            .set(item)
            .returning(Service::as_returning())
            .get_result::<Service>(db)
            .map(|s| s.to_common_svc_config())
    }

    /// Deletes a service from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    /// * `param_service_id` - the ID of the service to delete
    ///
    /// # Returns
    ///
    /// A `QueryResult<usize>` containing the number of rows deleted,
    /// or an error if the operation fails.
    ///
    pub fn delete(db: &mut Connection, param_service_id: i32) -> QueryResult<usize> {
        diesel::delete(service.filter(service_id.eq(param_service_id))).execute(db)
    }
}
