use crate::TraitDataIntegration;
use std::fmt::Error;

pub struct MockDataIntegration;

impl TraitDataIntegration for MockDataIntegration {
    async fn run(&self) -> Result<u32, Error> {
        Ok(20)
    }
}
