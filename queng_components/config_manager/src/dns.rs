use crate::CfgManager;

impl CfgManager {
    /**
     * Returns a reference to the IP address of the internal DNS resolver.
     */
    pub fn internal_dns_server(&self) -> &str {
        &self.internal_dns_server
    }

    /**
     * Returns a reference to the IP address of the external DNS resolver.
     */
    pub fn external_dns_server(&self) -> &str {
        &self.external_dns_server
    }
}
