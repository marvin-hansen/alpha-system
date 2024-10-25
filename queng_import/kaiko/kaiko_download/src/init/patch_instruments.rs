use crate::init::patches::EXCHANGE_INSTRUMENT_MODIFICATIONS;
use crate::init::InitManager;
use common_metadata::prelude::MetaInstrument;

impl InitManager {
    ///
    /// Patch instruments based on specific modifications for certain exchange codes and pair codes.
    ///
    /// This function takes a `MetaInstrument` as input, applies modifications based on the exchange code and pair code,
    /// and returns the patched `MetaInstrument`.
    ///
    /// If modifications are found for the provided exchange code and pair code,
    /// the function updates the `base_asset` or `class` fields accordingly.
    ///
    /// The modifications are specific to certain exchange codes and pair codes.
    ///
    /// # Arguments
    /// - `meta_instrument`: The original `MetaInstrument` to be patched.
    ///
    /// # Returns
    /// A patched `MetaInstrument` with updated fields based on the modifications.
    ///
    pub(crate) fn patch_instruments(&self, meta_instrument: MetaInstrument) -> MetaInstrument {
        let mut patch_instrument = meta_instrument.clone();

        if let Some(modifications) =
            EXCHANGE_INSTRUMENT_MODIFICATIONS.get(patch_instrument.exchange_code.as_str())
        {
            if let Some(value) = modifications.get(patch_instrument.exchange_pair_code.as_str()) {
                match patch_instrument.exchange_code.as_str() {
                    "okex" => {
                        if patch_instrument.exchange_pair_code.as_str() == "NEIROETH-USDT-SWAP" {
                            patch_instrument.base_asset = value.to_string();
                        }
                    }
                    _ => {
                        patch_instrument.class = value.to_string();
                    }
                }
            }
        }

        self.print_instrument("Done! Patched instrument: {:?}", &patch_instrument);

        patch_instrument
    }

    fn print_instrument(&self, msg: &str, meta_instrument: &MetaInstrument) {
        if self.dbg {
            println!();
            println!("{}: {:?}", msg, meta_instrument);
            println!();
        }
    }
}
