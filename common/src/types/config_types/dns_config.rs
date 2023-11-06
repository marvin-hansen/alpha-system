use std::fmt::{Display, Formatter};

pub struct DnsConfig<'l> {
    dns_host_external: &'l str,
    dns_port_external: &'l str,
    dns_host_internal: &'l str,
    dns_port_internal: &'l str,
}

impl<'l> DnsConfig<'l> {
    pub fn new(
        dns_host_external: &'l str,
        dns_pot_external: &'l str,
        dns_host_internal: &'l str,
        dns_pot_internal: &'l str,
    ) -> Self {
        Self {
            dns_host_external,
            dns_port_external: dns_pot_external,
            dns_host_internal,
            dns_port_internal: dns_pot_internal,
        }
    }
}

impl<'l> DnsConfig<'l> {
    pub fn dns_host_external(&self) -> &'l str {
        self.dns_host_external
    }
    pub fn dns_port_external(&self) -> &'l str {
        self.dns_port_external
    }
    pub fn dns_host_internal(&self) -> &'l str {
        self.dns_host_internal
    }
    pub fn dns_port_internal(&self) -> &'l str {
        self.dns_port_internal
    }
}

impl<'l> Default for DnsConfig<'l> {
    fn default() -> Self {
        Self {
            dns_host_external: "1.1.1.1",
            dns_port_external: "53",
            dns_host_internal: "", // determined dynamically based on context
            dns_port_internal: "53",
        }
    }
}

impl<'l> Display for DnsConfig<'l> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "DnsConfig {{ dns_host_external: {}, dns_port_external: {}, dns_host_internal: {}, dns_port_internal: {} }}",
               self.dns_host_external, self.dns_port_external, self.dns_host_internal, self.dns_port_internal
        )
    }
}
