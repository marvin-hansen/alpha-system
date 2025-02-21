/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::CfgManager;
use hickory_resolver::TokioAsyncResolver;
use hickory_resolver::error::ResolveError;
use std::net::IpAddr;

impl CfgManager {
    /**
     * Resolves a hostname using the appropriate DNS resolver (internal or external).
     */
    pub async fn resolve_dns(&self, host: &str, internal: bool) -> Result<IpAddr, ResolveError> {
        self.dbg_print("resolve_dns");
        self.dbg_print("internal");
        self.dbg_print(internal.to_string().as_str());
        self.dbg_print("host");
        self.dbg_print(host);

        if internal {
            resolve_address(&self.internal_dns_resolver, host).await
        } else {
            resolve_address(&self.external_dns_resolver, host).await
        }
    }
}

/**
 * Resolves a hostname using the specified DNS resolver.
 */
pub async fn resolve_address(
    resolver: &TokioAsyncResolver,
    host: &str,
) -> Result<IpAddr, ResolveError> {
    // resolve host asynchronously
    // https://docs.rs/hickory-resolver/latest/hickory_resolver/#using-the-tokioasync-resolver
    let response = match resolver.lookup_ip(host).await {
        Ok(response) => response,
        Err(e) => return Err(e),
    };

    match response.iter().next() {
        Some(address) => Ok(address),
        None => Err(ResolveError::from("Address not found")),
    }
}
