use kaiko_client;
use kaiko_client::KaikoClient;
use std::collections::HashSet;
use std::error::Error;

const API_URL: &str = "http://localhost:7777/";

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let client = KaikoClient::with_url(API_URL).expect("Failed to build Kaiko client");

    let instruments = client
        .get_instruments()
        .await
        .expect("Failed to get instruments from API")
        .data;

    println!("Got {} instruments", instruments.len());

    // HashSet to store only unique instrument codes.
    let mut symbol_codes = HashSet::new();

    for i in instruments.iter() {
        symbol_codes.insert(i.code.clone());
    }

    println!("Got {} unique symbols", symbol_codes.len());

    for s in symbol_codes {
        println!("{s}");
    }

    Ok(())
}
