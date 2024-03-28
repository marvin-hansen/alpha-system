use crate::SystemDBManager;

impl SystemDBManager {
    pub fn build_get_all_services_query(&self, service_table: &str) -> String {
        format!("SELECT * FROM {}", service_table)
    }
}
