use common::prelude::DnsConfig;

#[test]
fn test_new() {
    let dns_host_external = "1.1.1.1";
    let dns_port_external = ":53";
    let dns_host_internal = "";
    let dns_port_internal = ":53";

    let config = DnsConfig::new(dns_host_external, dns_port_external, dns_host_internal, dns_port_internal);
    assert_eq!(config.dns_host_external(), dns_host_external);
    assert_eq!(config.dns_port_external(), dns_port_external);
    assert_eq!(config.dns_host_internal(), dns_host_internal);
    assert_eq!(config.dns_port_internal(), dns_port_internal);
}

#[test]
fn test_default() {
    let config = DnsConfig::default();
    assert_eq!(config.dns_host_external(), "1.1.1.1");
    assert_eq!(config.dns_port_external(), ":53");
    assert_eq!(config.dns_host_internal(), "");
    assert_eq!(config.dns_port_internal(), ":53");
}

#[test]
fn test_display() {
    let dns_host_external = "1.1.1.1";
    let dns_port_external = ":53";
    let dns_host_internal = "";
    let dns_port_internal = ":53";

    let config = DnsConfig::new(dns_host_external, dns_port_external, dns_host_internal, dns_port_internal);
    assert_eq!(format!("{}", config), format!("DnsConfig {{ dns_host_external: {}, dns_port_external: {}, dns_host_internal: {}, dns_port_internal: {} }}", dns_host_external, dns_port_external, dns_host_internal, dns_port_internal));
}