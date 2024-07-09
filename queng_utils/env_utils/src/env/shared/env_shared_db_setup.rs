use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use clickhouse_utils::ClickhouseUtil;
use common::prelude::ContainerConfig;
impl EnvUtil {
    /// Configures the Clickhouse database for the environment.
    ///
    /// This method configures the Clickhouse database for the environment by performing the following steps:
    ///
    /// 1. Retrieves a new `ClickhouseUtil` object asynchronously.
    ///
    /// 2. Retrieves the Kaiko utility.
    ///
    /// 3. Creates all Clickhouse databases if they do not already exist using the `setup_all_db` method of the `ClickhouseUtil` object.
    ///
    /// # Arguments
    ///
    /// * `container_config` - A reference to the `ContainerConfig` object containing configuration details.
    /// * `sample_size` - An optional `u32` value representing the sample size.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the Clickhouse database is configured successfully, or an `Err` variant of `EnvironmentError` if an error occurs during the configuration process.
    ///
    /// # Errors
    ///
    /// Returns an `EnvironmentError` if any of the following errors occur during the configuration process:
    ///
    /// - If there is an error retrieving the Clickhouse utility.
    /// - If there is an error retrieving the Kaiko utility.
    /// - If there is an error creating the Clickhouse databases.
    ///
    pub(crate) async fn configure_clickhouse(
        &self,
        container_config: &ContainerConfig<'_>,
        sample_size: Option<u32>,
    ) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[setup_ci]: Get clickhouse utils");
        self.dbg_print("Get Clickhouse util");
        let ch_utils = &self
            .get_new_clickhouse_util()
            .await
            .expect("Failed to get ClickhouseUtil");

        self.dbg_print("Get Kaiko util");
        let kaiko_util = self.kaiko_util();

        self.dbg_print(
            "[configure_clickhouse]: Create all clickhouse databases if not already exist",
        );
        ch_utils
            .setup_all_db()
            .await
            .expect("Failed to create databases");

        self.dbg_print("[configure_clickhouse]: Check if clickhouse is already configured");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to check if all database tables configured");

        self.dbg_print(&format!(
            "[configure_clickhouse]: Clickhouse is already configured: {}",
            tables_created
        ));

        if !tables_created {
            ch_utils
                .setup_all_tables()
                .await
                .expect("Failed to create metadata tables");
        };

        self.dbg_print("[configure_clickhouse]: Check if all clickhouse data are already imported");
        let data_imported = self
            .check_if_meta_data_imported(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to check if all data imported");

        self.dbg_print(&format!(
            "[configure_clickhouse]: Clickhouse data already imported: {}",
            data_imported
        ));
        self.dbg_print("[configure_clickhouse]: Check whether to keep existing CH config");
        let keep_data = container_config.keep_configuration();
        self.dbg_print(&format!(
            "[configure_clickhouse]: Keep Clickhouse config and data: {}",
            keep_data
        ));

        if tables_created && data_imported && keep_data {
            // If so, abort & return. Nothing to do in this case.
            self.dbg_print("[configure_clickhouse]: Nothing to configure or import; return.");

            return Ok(());
        }

        // Check if the container configuration should be reset. If so, delete everything.
        self.dbg_print("[configure_clickhouse]: Check if reset is required if data are outdated");
        if container_config.keep_configuration() {
            self.dbg_print("[configure_clickhouse]: Drop all databases");
            ch_utils
                .teardown_all_db()
                .await
                .expect("[configure_clickhouse]: Failed to drop all databases")
        }

        // We know that the DB is either not configured or has been deleted
        // so we can re-crete all databases, tables, and import all data;
        self.dbg_print("[configure_clickhouse]: Create all databases");
        self.setup_db(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all databases");

        self.dbg_print("[configure_clickhouse]: Create all tables");
        self.create_tables(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to create all tables");

        self.dbg_print("[configure_clickhouse]: Verify that all tables are created");
        let tables_created = self
            .verify_tables_created(ch_utils)
            .await
            .expect("[configure_clickhouse]: Failed to verify if all tables are created");
        assert!(tables_created);

        self.dbg_print("[configure_clickhouse]: Import data into clickhouse");
        self.import_metadata(ch_utils, kaiko_util, sample_size)
            .await
            .expect("[configure_clickhouse]: Failed to import data into Clickhouse");
        Ok(())
    }

    /// Sets up the ClickHouse database for Continuous Integration (CI) testing.
    ///
    /// This function creates all databases required for testing.
    ///
    /// # Errors
    ///
    /// - `EnvironmentError` if any step fails.
    ///
    async fn setup_db(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[setup_db]: Create all databases");
        ch_utils
            .setup_all_db()
            .await
            .expect("[setup_db]: Failed to create all databases");

        Ok(())
    }

    async fn create_tables(&self, ch_utils: &ClickhouseUtil) -> Result<(), EnvironmentError> {
        //
        self.dbg_print("[create_tables]:Create all metadata tables");
        ch_utils
            .metadata
            .create_all_metadata_tables()
            .await
            .expect("[create_tables]: Failed to create metadata tables");

        Ok(())
    }
}
