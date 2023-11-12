use common::prelude::{MemGraphError, ServiceConfig, ServiceID};

#[derive(Debug, Default)]
pub struct SmdbDataManager {}

impl SmdbDataManager {
    /// Creates a new SmdbDataManager and connects to the MEMGRAPH database
    pub fn new() -> Self {
        Self {}
    }
}

impl SmdbDataManager {
    /// Closes the connection
    pub fn close_connection(&mut self) -> Result<(), MemGraphError> {
        Ok(())
    }

    /// Gets all services from the data store.
    pub fn get_all_services(&self) -> Result<Vec<ServiceConfig>, MemGraphError> {
        let services: Vec<ServiceConfig> = Vec::new();
        Ok(services)
    }

    /// Gets a service from the data store.
    pub fn get_service(&self, _service_id: &ServiceID) -> Result<ServiceConfig, MemGraphError> {
        let service: ServiceConfig = ServiceConfig::default();
        Ok(service)
    }

    /// Checks if all dependencies for a service are met.
    pub fn check_all_service_dependencies(
        &self,
        _service_id: &ServiceID,
    ) -> Result<bool, MemGraphError> {
        Ok(false)
    }

    /// Checks if all services depend on a service.
    pub fn check_all_service_depends_on(
        &self,
        _service_id: &ServiceID,
    ) -> Result<bool, MemGraphError> {
        Ok(false)
    }

    /// Gets all dependencies for a service.
    pub fn get_all_service_dependencies(
        &self,
        _service_id: &ServiceID,
    ) -> Result<Vec<ServiceID>, MemGraphError> {
        let dependencies: Vec<ServiceID> = Vec::new();
        Ok(dependencies)
    }

    /// Gets all services that depend on a service.
    pub fn get_all_service_depends_on(
        &self,
        _service_id: &ServiceID,
    ) -> Result<Vec<ServiceID>, MemGraphError> {
        let dependencies: Vec<ServiceID> = Vec::new();
        Ok(dependencies)
    }

    /// Creates a service in the data store.
    pub fn create_service(&mut self, _service_config: &ServiceConfig) -> Result<(), MemGraphError> {
        Ok(())
    }

    /// Checks if a service exists in the data store.
    pub fn check_service_exists(&self, _service_id: &ServiceID) -> Result<bool, MemGraphError> {
        Ok(false)
    }

    /// Checks if a service is online.
    pub fn check_service_online(&self, _service_id: &ServiceID) -> Result<bool, MemGraphError> {
        Ok(false)
    }

    /// Deletes a service from the data store.
    pub fn delete_service(&self, _service_id: &ServiceID) -> Result<(), MemGraphError> {
        Ok(())
    }

    /// Registers a service with the data manager.
    pub fn register_service(&self, _service: &ServiceID) -> Result<(), MemGraphError> {
        Ok(())
    }

    /// Deregisters a service with the data manager.
    pub fn deregister_service(&self, _service: &ServiceID) -> Result<(), MemGraphError> {
        Ok(())
    }
}
