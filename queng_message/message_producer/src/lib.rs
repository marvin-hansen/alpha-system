mod getters;
mod produce;
mod shutdown;

use ahash::AHashMap;
use common_message::StreamUser;
use iggy::client::{Client, StreamClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::models::permissions::{Permissions, StreamPermissions};
use iggy::models::user_status::UserStatus;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

pub struct MessageProducer {
    user_id: Identifier,
    stream_id: Identifier,
    topic_id: Identifier,
    client: IggyClient,
    producer: IggyProducer,
}

impl MessageProducer {
    /// Creates a new `MessageProducer` instance using the provided credentials and identifiers.
    ///
    /// # Arguments
    ///
    /// * `username` - The username for stream authentication.
    /// * `password` - The password for stream authentication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    /// * `tcp_server_address` - The tcp server address i.e. "127.0.0.1:8090"
    ///
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn new(
        stream_id: String,
        topic_id: String,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(args, stream_user).await
    }

    /// Creates a new `MessageProducer` instance using the provided `ImsDataConfig`.
    ///
    /// # Arguments
    ///
    /// * `config` - The `ImsDataConfig` to build the `MessageProducer` instance from.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn from_config(
        config: &common_message::ImsDataConfig,
        stream_user: &StreamUser,
    ) -> Result<Self, IggyError> {
        let args = Args::from_ims_data_config(config);
        Self::build(args, stream_user).await
    }

    /// Creates a new `MessageProducer` instance using the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        Self::build(Args::default(), &StreamUser::default()).await
    }
}

impl MessageProducer {
    async fn build(args: Args, stream_user: &StreamUser) -> Result<Self, IggyError> {
        // Build client
        let client = shared_utils::build_client(args.to_sdk_args())
            .await
            .expect("Failed to create client");

        // Connect client
        client.connect().await.expect("Failed to connect");

        // Create producer
        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)
            .expect("Failed to create producer")
            .batch_size(args.messages_per_batch)
            .send_interval(IggyDuration::from_str(&args.interval).expect("Invalid interval format"))
            .partitioning(Partitioning::balanced())
            .build();

        // Create stream
        let stream = client
            .create_stream(&args.stream_id, None)
            .await
            .expect("Failed to create stream");

        // Configure stream permissions
        let mut streams_permissions = AHashMap::new();
        streams_permissions.insert(
            stream.id,
            StreamPermissions {
                read_stream: true,
                read_topics: true,
                ..Default::default()
            },
        );

        let permissions = Permissions {
            streams: Some(streams_permissions),
            ..Default::default()
        };

        // Create custom stream user
        client
            .create_user(
                &stream_user.username(),
                &stream_user.password(),
                UserStatus::Active,
                Some(permissions),
            )
            .await
            .expect("Failed to create user");

        // Init producer
        producer.init().await.expect("Failed to init producer");

        // Create identifiers for stream, topic, and user.
        let stream_id = Identifier::from_str_value(&args.stream_id).expect("Invalid stream id");
        let topic_id = Identifier::from_str_value(&args.topic_id).expect("Invalid topic id");
        let user_id = Identifier::from_str_value(&args.username).expect("Invalid user id");

        Ok(Self {
            user_id,
            stream_id,
            topic_id,
            client,
            producer,
        })
    }
}
