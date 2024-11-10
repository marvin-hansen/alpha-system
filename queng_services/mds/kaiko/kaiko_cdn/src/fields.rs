// Binding to the cloudflare KV namespace
pub(crate) const METADATA_KV: &str = "METADATA";
pub(crate) const ASSETS_KEY: &str = "assets";
pub(crate) const EXCHANGES_KEY: &str = "exchanges";
pub(crate) const INSTRUMENTS_KEY: &str = "instruments";
pub(crate) const STATS_KEY: &str = "stats";

// Fields for authentication
pub(crate) const AUTH_HEADER_KEY: &str = kaiko_cdn_auth::AUTH_HEADER_KEY;
pub(crate) const RO_AUTH_KEY: &str = kaiko_cdn_auth::RO_AUTH_KEY;
pub(crate) const RW_AUTH_KEY: &str = kaiko_cdn_auth::RW_AUTH_KEY;
