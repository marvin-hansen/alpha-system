use crate::ServiceImportManager;
use std::io::{Error, ErrorKind};

impl ServiceImportManager {
    /// Checks if all services have already been imported into the database.
    ///
    /// Returns `true` if the number of services matches the database count.
    ///
    pub async fn check_if_already_imported(&self) -> bool {
        self.dbg_print("Check if already imported");

        let expected_count = self.services.len();
        // Count if there is any service already in the database
        let actual_count = self.count_db_services().await;

        // If all services have already been imported, return true
        // Note, the simple count works because each service is unique with an unique primary key.
        actual_count == expected_count
    }

    /// Asynchronously imports services into the database, ensuring all services are present.
    /// Sets up necessary databases and handles importing new or missing services.
    ///
    pub async fn import_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.dbg_print("import_services");

        // Get the expected number of services
        let expected_count = self.services.len();

        // Count if there is any service already in the database
        let actual_count = self.count_db_services().await;

        // If all services have already been imported, exit the program
        if actual_count == expected_count {
            return Ok(());
        }

        // If nothing has been imported yet, import all services
        if actual_count == 0 {
            self.dbg_print("Import all services");
            self.dbm_smdb
                .insert_service_collection(&self.services)
                .await
                .expect("Failed to import services");

            // Count all imported services
            let post_import_count = self.count_db_services().await;

            // Check if all services have been imported
            if post_import_count != expected_count {
                self.dbg_print("Failed to import all services. Check database records manually to determine missing services");
                return Err(Box::new(Error::from(ErrorKind::Other)));
            }

            return Ok(());
        }

        // If some services have been already imported, yet some new new have been added,
        // so we have to check one by one to test which one to add.
        if actual_count > 0 && actual_count < expected_count {
            for s in &self.services {
                let id = s.svc_id();
                let exists = self
                    .dbm_smdb
                    .check_if_service_id_exists(id)
                    .await
                    .expect("Failed to check if service exists");

                if !exists {
                    self.dbg_print(&format!("Importing service: {}", id));
                    self.dbm_smdb
                        .insert_service(s)
                        .await
                        .expect("Failed to import service");
                }
            }
        }

        Ok(())
    }

    /// Asynchronously counts the number of services in the database.
    ///
    /// # Returns
    ///
    /// The total number of services as a `usize`.
    ///
    /// # Panics
    /// Panics if the service count operation fails.
    pub async fn count_db_services(&self) -> usize {
        self.dbg_print("count_db_services");

        self.dbm_smdb
            .count_services()
            .await
            .expect("Failed to count services") as usize
    }
}
