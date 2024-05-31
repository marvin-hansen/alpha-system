use common::prelude::{Instrument, InstrumentMapping, SymbolMapping};
use kaiko_client;
use kaiko_client::KaikoClient;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;

const API_URL: &str = "http://localhost:7777/";

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    //
    let client = KaikoClient::with_url(API_URL).expect("Failed to build Kaiko client");

    let instruments = client
        .get_instruments()
        .await
        .expect("Failed to get instruments from API")
        .data;

    println!("Got {} instruments", instruments.len());

    // HashSet to store only unique instrument codes.
    let mut symbol_codes = HashSet::new();
    let mut etf = HashSet::new();
    let mut perpetual = HashSet::new();
    let mut spot = HashSet::new();

    for i in instruments.iter() {
        symbol_codes.insert(i.code.clone());

        if i.class.contains("etf") {
            etf.insert(i.code.clone());
        }

        if i.class.contains("spot") {
            spot.insert(i.code.clone());
        }

        if i.class.contains("perpetual-future") {
            perpetual.insert(i.code.clone());
        }
    }

    println!("Got {} unique symbols", symbol_codes.len());
    println!("Got {} unique ETF symbols", etf.len());
    println!("Got {} unique PERP symbols", perpetual.len());
    println!("Got {} unique SPOT symbols", spot.len());

    // Symbol code <==> Symbol Mapping
    let mut data: BTreeMap<String, SymbolMapping> = BTreeMap::new();

    // Sort symbols
    let mut symbols: Vec<String> = symbol_codes.into_iter().collect();
    symbols.sort();

    for s in symbols.iter() {
        let mut mapping: HashMap<String, InstrumentMapping> = HashMap::new();
        let mut instrument = Instrument::default();

        println!("{:?}", s);
        for i in instruments.iter() {
            if s.eq_ignore_ascii_case(&i.code) {
                println!("{:?}", i);
                println!();

                let im = InstrumentMapping::new(
                    i.exchange_code.to_owned(),
                    i.exchange_pair_code.to_owned(),
                    i.clone().metadata.unwrap_or_default().instrument_figi,
                    i.trade_count,
                );

                instrument = i.clone();
                mapping.insert(i.exchange_code.to_owned(), im);
            }
        }

        let sm = SymbolMapping::new(
            instrument.code.to_owned(),
            instrument.class.to_owned(),
            instrument.clone().metadata.unwrap_or_default().pair_figi,
            instrument.base_asset.to_owned(),
            instrument.quote_asset.to_owned(),
            mapping,
        );

        data.insert(s.to_owned(), sm);

        println!();
        println!("==========================");
        println!();
    }

    println!("Got {} unique mappings", data.len());
    for sm in data.iter() {
        println!("==========================");
        println!();
        println!("{:?}", sm);
        println!();
    }

    Ok(())
}
