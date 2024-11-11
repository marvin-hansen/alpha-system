use crate::prelude::ImsIntegrationType;
pub use common_exchange::prelude::ExchangeID;

pub struct ImsIntegration {
    integration_id: String,
    ims_integration_type: ImsIntegrationType,
    exchange_id: ExchangeID,
}
