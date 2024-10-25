use lazy_static::lazy_static;
use std::collections::HashMap;

const SPOT: &str = "spot";
const USDC: &str = "usdc";
const LUNAC: &str = "lunac";

lazy_static! {

    /// 'EXCHANGE_INSTRUMENT_MODIFICATIONS' defines a static lazy-loaded HashMap containing modifications for exchange instruments.
    /// The HashMap stores mappings for exchange symbols to specific instrument types.
    /// This structure is used to efficiently manage and access instrument modifications for various exchanges.
    /// The key-value pairs represent exchange symbols and their corresponding instrument types.
    /// This hash map is crucial for optimizing the handling of exchange instrument data within the application.
    /// https://docs.rs/lazy_static/latest/lazy_static/
    pub(crate) static ref EXCHANGE_INSTRUMENT_MODIFICATIONS: HashMap::<&'static str, HashMap<&'static str, &'static str>> = {
        let mut exchange_pair_modifications = HashMap::<&str, HashMap<&str, &str>>::new();

        exchange_pair_modifications.insert("bbit", {
            let mut map = HashMap::<&str, &str>::new();
            map.insert("POLUSDT", SPOT);
            map.insert("POLPERP", USDC);
            map
        });
        exchange_pair_modifications.insert("bgdm", {
            let mut map = HashMap::<&str, &str>::new();
            map.insert("POLUSDT", SPOT);
            map.insert("LUNCUSDT", LUNAC);
            map.insert("BTCPERP", USDC);
            map.insert("ETHPERP", USDC);
            map
        });
        exchange_pair_modifications.insert("hbdm", {
            let mut map = HashMap::<&str, &str>::new();
            map.insert("peopleusdt", SPOT);
            map
        });
        exchange_pair_modifications.insert("okex", {
            let mut map = HashMap::<&str, &str>::new();
            map.insert("NEIROETH-USDT-SWAP", "neiroeth");
            map
        });

        exchange_pair_modifications
    };
}
