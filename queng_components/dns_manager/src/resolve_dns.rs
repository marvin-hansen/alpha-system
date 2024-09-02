use crate::DnsManager;
use hickory_resolver::error::ResolveError;
use hickory_resolver::Resolver;
use std::net::IpAddr;

impl DnsManager {
    /**
     * Resolves a hostname using the appropriate DNS resolver (internal or external).
     */
    pub fn resolve_dns(&self, host: &str, internal: bool) -> Result<IpAddr, ResolveError> {
        if internal {
            resolve_address(&self.internal_dns_resolver, host)
        } else {
            resolve_address(&self.external_dns_resolver, host)
        }
    }
}

/**
 * Resolves a hostname using the specified DNS resolver.
 */
fn resolve_address(resolver: &Resolver, host: &str) -> Result<IpAddr, ResolveError> {
    // resolve host name using the external DNS server
    let response = match resolver.lookup_ip(host) {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    match response.iter().next() {
        Some(address) => Ok(address),
        None => Err(ResolveError::from("Address not found")),
    }
}
