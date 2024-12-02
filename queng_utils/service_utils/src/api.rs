use crate::error::service_util_error::ServiceUtilError;
use crate::fields::PATH;
use crate::{ServiceUtil, ServiceWaitStrategy};
use common_config::ServiceID;
use common_exchange::ExchangeID;
use common_ims::ImsIntegrationType;

impl ServiceUtil {
    /// Starts a specified service using the provided service ID and wait strategy.
    ///
    /// # Arguments
    ///
    /// * `svc` - A reference to the `ServiceID` representing the service to be started.
    /// * `wait_strategy` - A reference to the `ServiceWaitStrategy` to determine how long to wait for the service to start.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the service starts successfully.
    /// * `Err(ServiceUtilError)` if there is an error starting the service.
    ///
    pub async fn start_service(
        &self,
        svc: &ServiceID,
        wait_strategy: &ServiceWaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        self.dbg_print("start_service");
        self.dbg_print(&format!(
            "Starting service: {}",
            svc.to_string().to_lowercase()
        ));

        self.dbg_print("start service");
        let program = format!("{}/{}", PATH, svc.to_string().to_lowercase());
        match self.start_program(program, wait_strategy).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Starts an IMS exchange integration using the provided exchange ID, integration type, and wait strategy.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange ID of the integration to be started.
    /// * `ims_integration_type` - The `ImsIntegrationType` of the integration to be started.
    /// * `wait_strategy` - A reference to the `ServiceWaitStrategy` to determine how long to wait for the integration to start.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the integration starts successfully.
    /// * `Err(ServiceUtilError)` if there is an error starting the integration.
    ///
    pub async fn start_ims_integration(
        &self,
        exchange_id: ExchangeID,
        ims_integration_type: ImsIntegrationType,
        wait_strategy: &ServiceWaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        self.dbg_print("start_ims_integration");

        let integration_id = format!(
            "{}_{}",
            exchange_id.to_string().to_lowercase(),
            ims_integration_type.to_string().to_lowercase()
        );
        self.dbg_print(&format!("Starting ims integration: {}", &integration_id,));

        self.dbg_print("start ims integration");
        let program = format!("{}/{}", PATH, integration_id);
        match self.start_program(program, wait_strategy).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
