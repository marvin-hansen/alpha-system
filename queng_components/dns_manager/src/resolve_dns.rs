use crate::DnsManager;
use hickory_resolver::error::ResolveError;
use hickory_resolver::Resolver;
use std::net::IpAddr;
use std::sync::Arc;

impl DnsManager {
    /**
     * Resolves a hostname using the appropriate DNS resolver (internal or external).
     */
    pub fn resolve_dns(&self, host: &str, internal: bool) -> Result<IpAddr, ResolveError> {
        let arc_key = Arc::new(host.to_string());

        return if internal {
            // Check cache
            if let Some(cached_ip_address) = self.internal_dns_cache.get(&arc_key) {
                return Ok(cached_ip_address);
            }

            // No cache hit; resolve host name using the internal (cluster) DNS server
            let resolved_ip_address = resolve_address(&self.internal_dns_resolver, host);

            // If ok, add result to cache
            if resolved_ip_address.is_ok() {
                let key = host.to_string();
                let value = resolved_ip_address.clone().unwrap();
                self.internal_dns_cache.insert(key, value)
            }
            resolved_ip_address
        } else {
            // Check cache
            if let Some(cached_ip_address) = self.external_dns_cache.get(&arc_key) {
                return Ok(cached_ip_address);
            }

            // No cache hit;  resolve host name using the external DNS server
            let resolved_ip_address = resolve_address(&self.external_dns_resolver, host);

            // If ok, add result to cache
            if resolved_ip_address.is_ok() {
                let key = host.to_string();
                let value = resolved_ip_address.clone().unwrap();
                self.external_dns_cache.insert(key, value)
            }

            resolved_ip_address
        };
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
