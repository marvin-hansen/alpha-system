use rsmgclient::{Connection, ConnectionStatus, ConnectParams};

use common::prelude::{MemGraphError, ServiceConfig, ServiceID};

// Implemented with
// memgraph client: https://memgraph.com/docs/client-libraries/rust
// cypher-dto https://crates.io/crates/cypher-dto/0.1.0

pub struct SmdbDataManager {
    connection: Connection,
}

impl SmdbDataManager {
    /// Creates a new SmdbDataManager and connects to the MEMGRAPH database
    pub fn new(connect_params: &ConnectParams) -> Self {

        // Connect to Memgraph
        let connection = Connection::connect(connect_params)
            .expect("[SmdbDataManager]: Failed to connect to Memgraph");

        // Check if connection is established.
        let status = connection.status();

        // Check if connection is ready.
        if status != ConnectionStatus::Ready {
            panic!("[SmdbDataManager]: Connection to Memgraph failed with status: {:?}", status);
        }

        Self {
            connection
        }
    }
}

impl SmdbDataManager {
    /// Returns the current connection status.
    pub fn get_connection_status(&self) -> Result<ConnectionStatus, MemGraphError> {
        Ok(self.connection.status())
    }

    /// Closes the connection 
    pub fn close_connection(&mut self) -> Result<(), MemGraphError> {
        self.connection.close();
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
    pub fn create_service(&self, _service_config: &ServiceConfig) -> Result<(), MemGraphError> {
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
