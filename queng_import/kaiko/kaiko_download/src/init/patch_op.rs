use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
#[warn(clippy::enum_variant_names)]
pub enum PatchOp {
    PatchBaseAsset,
    PatchClass,
    PatchQuoteAsset,
}

impl Display for PatchOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
