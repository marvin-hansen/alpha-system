use crate::prelude::{ContainerConfig, TestEnvError};

pub(crate) fn configure_reset_or_reuse_clickhouse_db(
    // client: &Client,
    container_config: &ContainerConfig,
) -> Result<(), TestEnvError> {
    // Check if DB is already configured
    let configured = is_clickhouse_configured(container_config);
    // Check if the container configuration should be re-set every time
    let reset_config = container_config.reset_configuration();

    // Container is *NOT* configured.
    if !configured {
        // Configure container from scratch.
        configure_clickhouse(false).expect("Failed to configure ClickHouse");
    }

    // Configuration needs to be reset.
    if reset_config {
        // Reset all configuration to its initial state
        configure_clickhouse(true).expect("Failed to reset and re-configure ClickHouse");
    }

    // Container is fully configured, no reset needed,
    // therefor its good to re-use. Just return OK.

    Ok(())
}

pub(crate) fn is_clickhouse_configured(_container_config: &ContainerConfig) -> bool {
    false
}

pub(crate) fn configure_clickhouse(_reset_config: bool) -> Result<(), TestEnvError> {
    Ok(())
}
