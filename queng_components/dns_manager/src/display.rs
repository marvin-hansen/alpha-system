use crate::DnsManager;
use std::fmt::{Display, Formatter};

impl Display for DnsManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DnsManager: \n env_type: {} \n internal_dns_server: {} \n external_dns_server {}",
            &self.env_type, &self.internal_dns_server, &self.external_dns_server,
        )
    }
}
