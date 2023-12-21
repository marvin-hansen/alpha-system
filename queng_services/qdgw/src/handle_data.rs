use crate::service::Server;
use common::prelude::{FileConfigType, MessageProcessingError, SymbolID};
use fluvio::{Fluvio, RecordKey};
use qd_manager::QDManager;
use sbe_messages::prelude::{
    LastDataBar, SbeDataBar, StartDataMessage, StopAllDataMessage, StopDataMessage,
};

impl Server {
    pub(crate) async fn start_data(
        &self,
        qd_manager: &QDManager,
        client_data_channel: &str,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::start_date]: start_data: {:?}",
            start_data_msg
        );

        let fluvio = Fluvio::connect().await.unwrap();

        let producer = fluvio
            .topic_producer(client_data_channel)
            .await
            .expect("Failed to create a producer");

        let symbol = start_data_msg.symbol_id().clone();

        if symbol == SymbolID::BTCUSD {
            let symbol = &FileConfigType::BtcSmall;

            let result = qd_manager.get_data_bars(symbol);

            assert!(result.is_ok());

            let bars = result.expect("get data bars failed");

            // Remove debug print
            println!(
                "[QDGW/handle::start_date]: sending # bars: {:?}",
                bars.len()
            );

            for bar in bars {
                let (_, buffer) =
                    SbeDataBar::encode_data_bar_message(bar).expect("Failed to encode bar");

                producer
                    .send(RecordKey::NULL, buffer)
                    .await
                    .expect("Failed to send Done!");
            }

            // Send last bar message to inform the client that the data stream has ended
            let last_bar = LastDataBar::new();
            let (_, buffer) = last_bar.encode().expect("Failed to encode last data bar");

            producer
                .send(RecordKey::NULL, buffer)
                .await
                .expect("Failed to send Done!");

            producer.flush().await.expect("Failed to flush");
        }

        Ok(())
    }

    pub(crate) async fn stop_date(
        &self,
        stop_data_msg: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg);

        Ok(())
    }

    pub(crate) async fn stop_all_data(
        &self,
        stop_all_data_msg: &StopAllDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::stop_all_data]: stop_all_data: {:?}",
            stop_all_data_msg
        );

        Ok(())
    }
}
