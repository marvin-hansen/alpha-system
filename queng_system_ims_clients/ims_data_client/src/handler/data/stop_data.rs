/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{ImsDataClient, ImsDataClientError};
use sbe_messages_control::StopDataMessage;
use sbe_types::DataType;

impl ImsDataClient {
    pub(crate) async fn client_stop_data(
        &self,
        symbol_id: String,
        data_type_id: DataType,
    ) -> Result<(), ImsDataClientError> {
        self.dbg_print("stop_data");

        self.dbg_print("Construct stop_data message");
        let stop_data_message =
            StopDataMessage::new(self.client_id, self.exchange_id, symbol_id, data_type_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = stop_data_message
            .encode()
            .expect("[ImsDataClient/stop_data]: Failed to encode message");

        self.dbg_print("Send stop_data message");
        self.send_one_message(message)
            .await
            .expect("[ImsDataClient/stop_data]: Failed to send stop_data message!");

        Ok(())
    }
}
