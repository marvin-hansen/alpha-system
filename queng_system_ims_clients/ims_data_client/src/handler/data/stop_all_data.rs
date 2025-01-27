use crate::{ImsDataClient, ImsDataClientError};
use sbe_messages_control::StopAllDataMessage;

impl ImsDataClient {
    pub(crate) async fn client_stop_all_data(&self) -> Result<(), ImsDataClientError> {
        self.dbg_print("stop_all_data");

        self.dbg_print("Construct stop_all_data message");
        let stop_all_data_message = StopAllDataMessage::new(self.client_id, self.exchange_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = stop_all_data_message
            .encode()
            .expect("[ImsDataClient/stop_all_data]: Failed to encode message");

        self.dbg_print("Send stop_all_data message");
        self.send_one_message(message)
            .await
            .expect("[ImsDataClient/stop_all_data]: Failed to send stop_all_data message!");

        Ok(())
    }
}
