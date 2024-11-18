use iggy::client::Client;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

mod getters;
mod shutdown;

pub struct MessageConsumer {
    user_id: Identifier,
    stream_id: Identifier,
    topic_id: Identifier,
    client: IggyClient,
    consumer: IggyConsumer,
}

impl MessageConsumer {
    /// Creates a new `MessageConsumer` instance using the provided credentials and identifiers.
    ///
    /// # Arguments
    ///
    /// * `consumer_name` - The name of the consumer.
    /// * `username` - The username for stream authentication.
    /// * `password` - The password for stream authentication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `tcp_server_address` - The tcp server address i.e. "127.0.0.1:8090"
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn new(
        consumer_name: &str,
        username: String,
        password: String,
        stream_id: String,
        topic_id: String,
        tcp_server_address: String,
    ) -> Result<Self, IggyError> {
        Self::build(
            Args::new(username, password, stream_id, topic_id, tcp_server_address),
            consumer_name,
        )
        .await
    }

    /// Creates a new `MessageConsumer` instance using the provided `ImsDataConfig`.
    ///
    /// # Arguments
    ///
    /// * `config` - The `ImsDataConfig` to build the `MessageConsumer` instance from.
    /// * `consumer_name` - The name of the consumer.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn from_config(
        config: &common_message::ImsDataConfig,
        consumer_name: &str,
    ) -> Result<Self, IggyError> {
        Self::build(Args::from_ims_data_config(config), consumer_name).await
    }

    /// Creates a new `MessageConsumer` instance using the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageConsumer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        let consumer_name = "default-message-consumer";
        Self::build(Args::default(), consumer_name).await
    }
}

impl MessageConsumer {
    async fn build(args: Args, consumer_name: &str) -> Result<Self, IggyError> {
        // Build client
        let client = shared_utils::build_client(args.to_sdk_args())
            .await
            .expect("Failed to create client");

        // Connect client
        client.connect().await.expect("Failed to connect");

        // Build consumer
        let mut consumer =
            match ConsumerKind::from_code(args.consumer_kind).expect("Invalid consumer kind") {
                ConsumerKind::Consumer => client
                    .consumer(
                        consumer_name,
                        &args.stream_id,
                        &args.topic_id,
                        args.partition_id,
                    )
                    .expect("Failed to create consumer"),
                ConsumerKind::ConsumerGroup => client
                    .consumer_group(consumer_name, &args.stream_id, &args.topic_id)
                    .expect("Failed to create consumer group"),
            }
            .auto_commit(AutoCommit::When(AutoCommitWhen::PollingMessages))
            .create_consumer_group_if_not_exists()
            .auto_join_consumer_group()
            .polling_strategy(PollingStrategy::next())
            .poll_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .batch_size(args.messages_per_batch)
            .build();

        consumer
            .init()
            .await
            .expect("Failed to initialize consumer");

        // Create identifiers for stream, topic, and user.
        let stream_id = Identifier::from_str_value(&args.stream_id).expect("Invalid stream id");
        let topic_id = Identifier::from_str_value(&args.topic_id).expect("Invalid topic id");
        let user_id = Identifier::from_str_value(&args.username).expect("Invalid user id");

        Ok(Self {
            user_id,
            stream_id,
            topic_id,
            client,
            consumer,
        })
    }
}
