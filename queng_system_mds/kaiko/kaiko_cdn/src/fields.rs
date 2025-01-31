/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

// Binding to the cloudflare KV namespace
pub const METADATA_KV: &str = "METADATA";
pub const ASSETS_KEY: &str = "assets";
pub const EXCHANGES_KEY: &str = "exchanges";
pub const INSTRUMENTS_KEY: &str = "instruments";
pub const STATS_KEY: &str = "stats";

// Fields for authentication
pub const AUTH_HEADER_KEY: &str = kaiko_cdn_auth::AUTH_HEADER_KEY;
pub const RO_AUTH_KEY: &str = kaiko_cdn_auth::RO_AUTH_KEY;
pub const RW_AUTH_KEY: &str = kaiko_cdn_auth::RW_AUTH_KEY;
