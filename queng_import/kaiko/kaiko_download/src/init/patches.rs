use crate::init::patches::PatchOp::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum PatchOp {
    PatchClass,
    PatchQuoteAsset,
    PatchBaseAsset,
}

const SPOT: &str = "spot";
const USDC: &str = "usdc";
const LUNAC: &str = "lunac";

pub(crate) const INSTRUMENT_PATCHES: [(&str, &str, PatchOp, &str); 7] = [
    // bbit
    ("bbit", "POLUSDT", PatchClass, SPOT), // bbit: Instrument_class should be SPOT https://www.bybit.com/en/trade/spot/POL/USDT
    ("bbit", "POLPERP", PatchQuoteAsset, USDC), // bbit: quote_asset should be  USDC https://www.bybit.com/trade/futures/usdc/POL-PERP
    // bgdm
    ("bgdm", "LUNCUSDT", PatchBaseAsset, LUNAC), // bgdm: base_asset should be LUNAC https://www.bitget.com/futures/usdt/LUNCUSDT
    ("bgdm", "BTCPERP", PatchQuoteAsset, USDC), // bgdm: quote_asset should be  USDC https://www.bitget.com/futures/usdc/BTCPERP
    ("bgdm", "ETHPERP", PatchQuoteAsset, USDC), // bgdm: quote_asset should be  USDC https://www.bitget.com/futures/usdc/ETHPERP
    // hbdm / HTX https://www.htx.com
    ("hbdm", "peopleusdt", PatchClass, SPOT), // hbdm: Instrument_class should be SPOT https://www.htx.com/trade/people_usdt/
    // Okex https://www.okx.com/
    ("okex", "NEIROETH-USDT-SWAP", PatchBaseAsset, "neiroeth"), //okex: base_asset should be NEIROETH  https://www.okx.com/trade-swap-strategy/neiroeth-usdt-swap
];
