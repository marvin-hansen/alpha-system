pub(crate) fn get_value_from_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(cluster_dns_server) => cluster_dns_server,
        Err(e) => {
            panic!(
                "{} {}",
                format_args!("[PostgresConfigManager]: Failed to read {key} environment variable. Ensure {key} is set:"),
                e
            );
        }
    }
}
