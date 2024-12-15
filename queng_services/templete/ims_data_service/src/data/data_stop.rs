use crate::service::Service;
use common_errors::MessageProcessingError;
use common_exchange::ExchangeID;
use sbe_types::DataErrorType;

impl Service {
    pub async fn stop_data(
        &self,
        client_id: u16,
        exchange_id: &ExchangeID,
        _symbols: &[String],
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        self.dbg_print("stop_data");

        // Verify data request
        match self.verify_data_request(client_id, exchange_id).await {
            Ok(_) => {}
            Err((error_type, err)) => return Err((error_type, err)),
        };

        // Stop data for symbols

        Ok(())
    }
}
