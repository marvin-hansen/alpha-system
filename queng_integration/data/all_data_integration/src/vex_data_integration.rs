use crate::DataIntegrationTrait;
use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};
use vex_data_integration::ImsVexDataIntegration;

const ID: &str = "VexDataIntegration";

#[derive(Debug, Clone, Copy)]
pub struct VexDataIntegration {
    integration: ImsVexDataIntegration,
}

impl Default for VexDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl VexDataIntegration {
    pub fn new() -> Self {
        let integration = ImsVexDataIntegration::new();
        Self { integration }
    }
}

impl DataIntegrationTrait for VexDataIntegration {
    async fn id(&self) -> Result<String, Error> {
        Ok(ID.to_string())
    }

    async fn start_date<P>(&self, symbols: &[String], processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        self.integration.start_date(symbols, processor).await
    }

    async fn stop_date(&self, symbols: &[String]) -> Result<(), Error> {
        self.integration.stop_date(symbols).await
    }

    async fn stop_all_date(&self) -> Result<(), Error> {
        self.integration.stop_all_date().await
    }
}
