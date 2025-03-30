/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{ImsDataClient, ImsDataClientError};
use iggy::client::{Client, StreamClient, TopicClient};
use iggy::identifier::Identifier;

impl ImsDataClient {
    pub async fn shutdown(&self) -> Result<(), ImsDataClientError> {
        self.dbg_print("shutdown");

        match self.control_client.shutdown().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(e.to_string())),
        }
    }

    pub async fn shutdown_and_delete(&self) -> Result<(), ImsDataClientError> {
        self.dbg_print("shutdown_and_delete");

        self.dbg_print("construct topic and stream identifiers");
        let control_stream_id =
            Identifier::from_str_value(&self.integration_config.control_channel())
                .expect("Failed to create Identifier");
        //
        let control_topic_id =
            Identifier::from_str_value(&self.integration_config.control_channel())
                .expect("Failed to create Identifier");

        self.dbg_print("delete_topic");
        self.control_client
            .delete_topic(&control_stream_id, &control_topic_id)
            .await
            .expect("Failed to delete topic");

        self.dbg_print("delete_stream");
        self.control_client
            .delete_stream(&control_topic_id)
            .await
            .expect("Failed to delete stream");

        self.dbg_print("shutdown");
        match self.control_client.shutdown().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(e.to_string())),
        }
    }
}
