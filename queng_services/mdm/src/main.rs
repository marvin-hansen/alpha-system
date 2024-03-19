use crate::service::MetaDataServer;

mod client;
mod service;

// MVP of loading and verifying symbols from Binance.

#[tokio::main]
async fn main() {
    println!("Hello, MDM!");

    println!("Building new service");
    let svc = MetaDataServer::new();

    println!("Update reference symbols from Binance");
    svc.update_reference_symbols()
        .await
        .expect("Failed to update symbols");

    let refs = svc.reference_symbols().await;
    let nr_refs = refs.len();
    println!("Pulled {} symbols", nr_refs);

    println!("Validate symbols");
    let symbols = vec!["ETHBTC".to_string()];

    svc.validate_symbols(&symbols)
        .await
        .expect("Failed to validate symbols");

    let nr = symbols.len();
    println!("Validated symbols: {}", nr)
}
