use crate::prelude::EnvironmentError;
use crate::EnvUtil;
use common_container::prelude::ContainerConfig;

const MTD: &str = "[configure_clickhouse]";

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

        let keep_data = container_config.keep_configuration();
        self.dbg_print(&format!("{MTD}: Keep Clickhouse database: {}", keep_data));

        if !keep_data {
            ch_utils
                .drop_all_db()
                .await
                .expect("Failed to drop all databases");
        }

        let db_exists = ch_utils
            .verify_all_db_exists()
            .await
            .expect("Failed to verify databases");

        self.dbg_print(&format!("{MTD}: Clickhouse database exists: {}", db_exists));

        if !db_exists {
            self.dbg_print("[configure_clickhouse]: Create all databases");
            ch_utils
                .setup_all_db()
                .await
                .expect("Failed to create databases");
        }

        self.dbg_print("[configure_clickhouse]: Check if all clickhouse data are already imported");
        let data_imported = self
            .verify_clickhouse_data_imported(ch_utils, kaiko_util, None)
            .await
            .expect("[configure_clickhouse]: Failed to check if all data imported");

        self.dbg_print(&format!("{MTD}: Data imported: {}", data_imported));

        if data_imported {
            // If so, abort & return. Nothing to do in this case.
            self.dbg_print(&format!("{MTD}: Nothing to configure or import; return."));

            return Ok(());
        }

        if !data_imported {
            self.dbg_print("[configure_clickhouse]: Import data into clickhouse");
            self.import_all_data(ch_utils, kaiko_util, sample_size)
                .await
                .expect("[configure_clickhouse]: Failed to import data into Clickhouse");
        }

        Ok(())
    }
}
