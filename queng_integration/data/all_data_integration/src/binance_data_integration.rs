use crate::TraitDataIntegration;
use std::fmt::Error;

pub struct BinanceDataIntegration;

impl TraitDataIntegration for BinanceDataIntegration {
    async fn run(&self) -> Result<u32, Error> {
        Ok(42)
    }
}
