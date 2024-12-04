use enum_dispatch::enum_dispatch;
use std::fmt::Error;

pub trait TradeProcessor {
    /// Callback to process a bar of trade data
    fn process_trade_bar(
        &self,
        trades: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

#[enum_dispatch(ImsDataIntegration)]
pub trait ImsDataIntegration {
    fn start_date(
        &self,
        data_id: &[String],
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn stop_date(
        &self,
        data_id: &[String],
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn stop_all_date(&self) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
