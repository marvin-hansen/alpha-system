use crate::init::patch_op::PatchOp::*;
use crate::init::patches::INSTRUMENT_PATCHES;
use crate::init::InitManager;
use common_metadata::MetaInstrument;

impl InitManager {
    ///
    /// Patch instruments based on specific modifications for certain exchange codes and pair codes.
    ///
    /// This function takes a `MetaInstrument` as input, applies modifications based on the exchange code and pair code,
    /// and returns the patched `MetaInstrument`.
    ///
    /// If modifications are found for the provided exchange code and pair code,
    /// the function updates the `base_asset`, `quote_asset`, or `class` fields accordingly.
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

        // 1. Check if the instrument's exchange code and pair code exist in the ERRATA_INSTRUMENT_ID array.
        let exchange_code = &meta_instrument.exchange_code;
        let pair_code = &meta_instrument.exchange_pair_code;
        for (exchange_code_to_patch, pair_code_to_patch, patch_op, value_to_patch) in
            INSTRUMENT_PATCHES.into_iter()
        {
            if exchange_code.eq(exchange_code_to_patch) && pair_code.eq(pair_code_to_patch) {
                // 2 If so, apply the PatchOp and the value to the patch_instrument as follows:
                //    - PatchInstrumentBaseAsset: patch_instrument.base_asset = value_to_patch
                //    - PatchInstrumentClass: patch_instrument.class = value_to_patch
                //    - PatchInstrumentQuoteAsset: patch_instrument.quote_asset = value_to_patch
                match patch_op {
                    PatchBaseAsset => {
                        patch_instrument.base_asset = value_to_patch.to_string();
                    }
                    PatchClass => {
                        patch_instrument.class = value_to_patch.to_string();
                    }
                    PatchQuoteAsset => {
                        patch_instrument.quote_asset = value_to_patch.to_string();
                    }
                }

                // 3. Print the patched instrument.
                self.print_instrument("Done! Patched instrument", &patch_instrument);

                break;
            }
        }

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
