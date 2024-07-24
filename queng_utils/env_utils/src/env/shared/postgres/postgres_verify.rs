use crate::prelude::EnvironmentError;
use crate::EnvUtil;

impl EnvUtil {
    pub(crate) fn verify_postgres_db(&self) -> Result<bool, EnvironmentError> {
        Err(EnvironmentError::new("Not implemented".to_string()))
    }

    pub(crate) async fn verify_postgres_data_imported(&self) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_postgres_data_imported");

        self.dbg_print("[verify_postgres_data_imported]: verify_service_data_imported");
        let service_imported = self
            .verify_service_data_imported()
            .await
            .expect("Failed to verify service data imported");

        self.dbg_print("[verify_postgres_data_imported]: verify_portfolio_data_imported");
        let portfolio_imported = self
            .verify_portfolio_data_imported()
            .await
            .expect("Failed to verify portfolio data imported");

        self.dbg_print(&format!(
            "[verify_postgres_data_imported]: service_imported: {}",
            service_imported
        ));
        self.dbg_print(&format!(
            "[verify_postgres_data_imported]: portfolio_imported: {}",
            portfolio_imported
        ));
        let all_imported = service_imported && portfolio_imported;

        Ok(all_imported)
    }

    async fn verify_service_data_imported(&self) -> Result<bool, EnvironmentError> {
        self.dbg_print("verify_service_data_imported");

        // let nr_services = get_all_service_specs().len() as u64;

        Err(EnvironmentError::new("Not implemented".to_string()))
    }

    async fn verify_portfolio_data_imported(&self) -> Result<bool, EnvironmentError> {
        Err(EnvironmentError::new("Not implemented".to_string()))
    }
}
