use std::fmt::Error;
use trait_data_integration::{EventProcessor, ImsDataIntegration};

#[derive(Debug, Clone, Copy)]
pub struct ImsVexDataIntegration;

impl Default for ImsVexDataIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsVexDataIntegration {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImsDataIntegration for ImsVexDataIntegration {
    async fn start_date<P>(&self, symbols: &[String], _processor: P) -> Result<(), Error>
    where
        P: EventProcessor + Send + Sync + 'static,
    {
        println!("ImsVexDataIntegration start_date");
        println!("Start data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_date(&self, symbols: &[String]) -> Result<(), Error> {
        println!("ImsVexDataIntegration stop_date");
        println!("Stop data for symbols: {:#?}", symbols);
        Ok(())
    }

    async fn stop_all_date(&self) -> Result<(), Error> {
        println!("ImsVexDataIntegration stop_all_date");

        Ok(())
    }
}
