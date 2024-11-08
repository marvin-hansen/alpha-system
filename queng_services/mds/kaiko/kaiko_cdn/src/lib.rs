mod fields;
mod handle_assets;
mod handle_exchanges;
mod handle_health;
mod handle_instruments;
mod handle_shared;
mod handle_stats;

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/health", handle_health::handle_get_health)
        .get_async("/stats", handle_stats::handle_get_stats)
        // assets metadata
        .get_async("/assets", handle_assets::handle_get_assets)
        .put_async("/assets", handle_assets::handle_put_assets)
        .post_async("/assets", handle_assets::handle_post_assets)
        // exchanges metadata
        .get_async("/exchanges", handle_exchanges::handle_get_exchanges)
        .put_async("/exchanges", handle_exchanges::handle_put_exchanges)
        .post_async("/exchanges", handle_exchanges::handle_post_exchanges)
        // instruments metadata
        .get_async("/instruments", handle_instruments::handle_get_instruments)
        .run(req, env)
        .await
}
