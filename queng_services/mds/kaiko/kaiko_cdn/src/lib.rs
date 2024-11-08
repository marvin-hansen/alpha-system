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
        //  health check
        // curl -H "Content-Type: application/json" --request GET 'http://localhost:8787/health'
        .get_async("/health", handle_health::handle_get_health)
        // Get assets metadata
        // curl -H "Content-Type: application/json" --request GET 'http://localhost:8787/assets'
        .get_async("/assets", handle_assets::handle_get_assets)
        //  Try to put your data in a file, say body.json and then use
        // curl -H "Content-Type: application/json" --request PUT --data @assets.json http://localhost:8787/assets
        .put_async("/assets", handle_assets::handle_put_assets)
        // curl -H "Content-Type: application/json" --request POST --data @assets.json http://localhost:8787/assets
        .post_async("/assets", handle_assets::handle_post_assets)
        // exchanges metadata
        // curl -H "Content-Type: application/json" --request GET 'http://localhost:8787/exchanges'
        .get_async("/exchanges", handle_exchanges::handle_get_exchanges)
        // curl -H "Content-Type: application/json" --request PUT --data @exchanges.json http://localhost:8787/exchanges
        .put_async("/exchanges", handle_exchanges::handle_put_exchanges)
        // curl -H "Content-Type: application/json" --request POST --data @exchanges.json http://localhost:8787/exchanges
        .post_async("/exchanges", handle_exchanges::handle_post_exchanges)
        // instruments metadata
        // curl -H "Content-Type: application/json" --request GET http://localhost:8787/instruments
        .get_async("/instruments", handle_instruments::handle_get_instruments)
        // curl -H "Content-Type: application/json" --request PUT --data @instruments.json http://localhost:8787/instruments
        .put_async("/instruments", handle_instruments::handle_put_instruments)
        // curl -H "Content-Type: application/json" --request POST --data @instruments.json http://localhost:8787/instruments
        .post_async("/instruments", handle_instruments::handle_post_instruments)
        // metadata stats
        // curl -H "Content-Type: application/json" --request GET 'http://localhost:8787/stats'
        .get_async("/stats", handle_stats::handle_get_stats)
        // curl -H "Content-Type: application/json" --request PUT --data @stats.json http://localhost:8787/stats
        .put_async("/stats", handle_stats::handle_put_stats)
        // curl -H "Content-Type: application/json" --request POST --data @stats.json http://localhost:8787/stats
        .post_async("/stats", handle_stats::handle_post_stats)
        .run(req, env)
        .await
}
