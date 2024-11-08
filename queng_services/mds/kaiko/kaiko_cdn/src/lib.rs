mod handle_assets_metadata;
mod handle_exchanges_metadata;
mod handle_health;
mod handle_instruments_metadata;
mod handle_shared;
mod handle_stats;

use serde::{Deserialize, Serialize};
use worker::*;

const ASSETS_KEY: &str = "assets";
const EXCHANGES_KEY: &str = "exchanges";
const INSTRUMENTS_KEY: &str = "instruments";
const STATS_KEY: &str = "stats";

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TestAsset {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/health", handle_health::handle_get_health)
        // assets metadata
        .get_async("/assets", handle_assets_metadata::handle_get_assets)
        .put_async("/assets", handle_assets_metadata::handle_put_assets)
        .post_async("/assets", handle_assets_metadata::handle_post_assets)
        // exchanges metadata
        .get_async(
            "/exchanges",
            handle_exchanges_metadata::handle_get_exchanges,
        )
        // instruments metadata
        .get_async(
            "/instruments",
            handle_instruments_metadata::handle_get_instruments,
        )
        // stats
        .get_async("/stats", handle_stats::handle_get_stats)
        .run(req, env)
        .await
}
