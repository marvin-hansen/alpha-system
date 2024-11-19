use crate::ConfigImportManager;
use std::io::{Error, ErrorKind};

impl ConfigImportManager {
    pub async fn import_portfolio_configs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.dbg_print("import_portfolio_configs");

        let expected_count = self.portfolio_configs.len();

        let actual_count = self
            .dbm
            .count_portfolio_configs()
            .await
            .expect("Failed to count portfolios") as usize;

        // If all portfolio configs have already been imported, return Ok
        if actual_count == expected_count {
            return Ok(());
        }

        // If nothing has been imported yet, import all portfolio configs
        if actual_count == 0 {
            self.dbg_print("Import all portfolios");
            self.dbm
                .insert_portfolio_config_collection(&self.portfolio_configs)
                .await
                .expect("Failed to import portfolios");

            let post_import_count = self
                .dbm
                .count_portfolio_configs()
                .await
                .expect("Failed to count portfolios") as usize;

            if post_import_count != expected_count {
                self.dbg_print("Failed to import all portfolios. Check database records manually to determine missing portfolios");
                return Err(Box::new(Error::from(ErrorKind::Other)));
            }
        }

        // If some portfolios have already been imported, yet some new have been added,
        // Check the collection one by one and add the missing ones.

        if actual_count > 0 && actual_count < expected_count {
            for p in &self.portfolio_configs {
                let id = p.portfolio_id() as u16;

                // Check if the portfolio already exists
                let exists = self
                    .dbm
                    .check_if_portfolio_id_exists(id)
                    .await
                    .expect("Failed to check if portfolio exists");

                // If the portfolio doesn't exist, import it
                if !exists {
                    self.dbg_print(&format!("Importing portfolio: {id}"));
                    self.dbm
                        .insert_portfolio_config(p)
                        .await
                        .expect("Failed to import portfolio");
                }
            }
        }

        Ok(())
    }

    pub async fn check_if_portfolios_imported(&self) -> bool {
        self.dbg_print("check_if_portfolios_already_imported");

        let expected_count = self.portfolio_configs.len();
        let actual_count = self.count_db_portfolios().await;

        // If all portfolios have already been imported, return true
        actual_count == expected_count
    }

    pub async fn count_db_portfolios(&self) -> usize {
        self.dbg_print("count_db_portfolios");
        self.dbm
            .count_portfolio_configs()
            .await
            .expect("Failed to count portfolios") as usize
    }
}
