use crate::IntegrationImportManager;
use std::io::{Error, ErrorKind};

impl IntegrationImportManager {
    /// Import integration configs
    ///
    /// # Description
    ///
    /// This function is responsible for importing integration configs from the
    /// `integration_configs` field of `IntegrationImportManager` into the database.
    ///
    /// If all integrations have already been imported, the function will return
    /// `Ok(())`.
    ///
    /// If nothing has been imported yet, the function will import all integrations
    /// at once.
    ///
    /// If some integrations have already been imported, yet some new have been
    /// added, the function will check the collection one by one and add the
    /// missing ones.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` -
    ///     `Ok(())` if the import was successful
    ///     `Err(Box<dyn std::error::Error>)` if the import failed
    ///
    pub async fn import_integration_configs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.dbg_print("import_integration_configs");

        let expected_count = self.integration_configs.len();

        let actual_count = self
            .dbm
            .count_integration_configs()
            .await
            .expect("Failed to count integrations") as usize;

        // If all integrations have already been imported, return Ok
        if actual_count == expected_count {
            return Ok(());
        }

        // If nothing has been imported yet, import all integrations
        if actual_count == 0 {
            self.dbg_print("Import all integrations");
            self.dbm
                .insert_integration_config_collection(&self.integration_configs)
                .await
                .expect("Failed to import integrations");

            let post_import_count =
                self.dbm
                    .count_integration_configs()
                    .await
                    .expect("Failed to count integrations") as usize;

            if post_import_count != expected_count {
                return Err(Error::new(ErrorKind::Other, "Failed to import integrations").into());
            }
        }

        // If some integrations have already been imported, yet some new have been added,
        // Check the collection one by one and add the missing ones.
        if actual_count > 0 && actual_count < expected_count {
            for integration_config in &self.integration_configs {
                let integration_id = integration_config.integration_id();

                let exists = self
                    .dbm
                    .check_if_integration_config_exists(integration_id.into())
                    .await
                    .expect("Failed to check if integration exists");

                if !exists {
                    self.dbg_print(&format!(
                        "Importing integration config: {}",
                        &integration_id
                    ));
                }
                self.dbm
                    .insert_integration_config(integration_config.to_owned())
                    .await
                    .expect("Failed to import integration");
            }
        }

        Ok(())
    }

    /// Check if integrations have already been imported
    ///
    /// # Returns
    ///
    /// * `bool` - True if all integrations have already been imported
    pub async fn check_if_integrations_imported(&self) -> bool {
        self.dbg_print("check_if_integrations_already_imported");

        let expected_count = self.integration_configs.len();
        let actual_count = self.count_db_integrations().await;

        // If all integrations have already been imported, return true
        actual_count == expected_count
    }

    /// Count the number of integrations in the database
    ///
    /// # Returns
    ///
    /// * `usize` - The number of integrations in the database
    pub async fn count_db_integrations(&self) -> usize {
        self.dbg_print("count_db_integrations");
        self.dbm
            .count_integration_configs()
            .await
            .expect("Failed to count integrations") as usize
    }
}
