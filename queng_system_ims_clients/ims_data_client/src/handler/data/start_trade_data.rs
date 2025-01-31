/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{ImsDataClient, ImsDataClientError};
use common_data_bar::TimeResolution;
use sbe_messages_control::StartDataMessage;
use sbe_types::DataType;

impl ImsDataClient {
    pub(crate) async fn client_start_trade_data(
        &self,
        symbol_id: String,
    ) -> Result<(), ImsDataClientError> {
        self.dbg_print("start_trade_data");

        self.dbg_print("Construct start_trade_data message");
        let start_data_message = StartDataMessage::new(
            self.client_id,
            self.exchange_id,
            symbol_id,
            TimeResolution::NoValue,
            DataType::TradeData,
        );

        self.dbg_print("Encode SBE message");
        let (_, message) = start_data_message
            .encode()
            .expect("[ImsDataClient/start_data]: Failed to encode start_trade_data message");

        self.dbg_print("Send start_data message");
        self.send_one_message(message)
            .await
            .expect("[ImsDataClient/start_data]: Failed to send start_trade_data message!");

        Ok(())
    }
}
