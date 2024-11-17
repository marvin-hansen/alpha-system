mod produce;

use iggy::client::{Client, StreamClient, TopicClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use message_shared::utils as shared_utils;
use message_shared::Args;
use std::str::FromStr;

pub struct MessageProducer {
    user_id: String,
    stream_id: String,
    topic_id: String,
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
        username: String,
        password: String,
        stream_id: String,
        topic_id: String,
        tcp_server_address: String,
    ) -> Result<Self, IggyError> {
        Self::build(Args::new(
            username,
            password,
            stream_id,
            topic_id,
            tcp_server_address,
        ))
        .await
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
    pub async fn from_config(config: &common_message::ImsDataConfig) -> Result<Self, IggyError> {
        Self::build(Args::from_ims_data_config(config)).await
    }

    /// Creates a new `MessageProducer` instance using the default configuration.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn default() -> Result<Self, IggyError> {
        Self::build(Args::default()).await
    }
}

// For now, the simplest approach would be to create 3 separate IggyProducer structs,
// as they share the same underlying IggyClient and are rather small types,
// exposing helpful methods to deal with a single topic. Of course,
// it's also possible (like you said), to use a single IggyProducer
// and invoke send_to method depending on the topic ID.
// I'm not sure, if there's any other way that we could implement
// a single producer dealing with N topics in a better way (would be happy to hear about any ideas in that matter) 🙂
//
// We have a support for personal access tokens - you can find all the methods,
// to create a PAT on behalf of the specific user, so for example,
// you can create unique user per stream or per topic (with granular permissions),
// and then create PAT per this user, and use login_with_personal_access_token()
// instead of regular login. Keep in mind, though, that at least for now,
// PAT doesn't support an internal permission scope etc.
// so it's always the same as for the user for whom it was created.
//
// Nothing I can think of right now, you could always enable server-side data encryption
// or even client-side data encryption with the unique encryption key per user (stream, topic w/e),
// but this will ofc result in less throughput.
// As long as the users have their permissions set properly, it should be fine 🙂
// https://discord.com/channels/1144142576266530928/1144142577369628684/1307612839569260544

impl MessageProducer {
    async fn build(args: Args) -> Result<Self, IggyError> {
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

        // Create stream and user
        shared_utils::create_stream_and_user(
            &args.stream_id,
            &args.username,
            &args.password,
            &client,
        )
        .await
        .expect("Failed to create stream and user");

        // Init producer
        producer.init().await.expect("Failed to init producer");

        // Extract identifiers
        let user_id = args.username;
        let stream_id = args.stream_id;
        let topic_id = args.topic_id;

        Ok(Self {
            user_id,
            stream_id,
            topic_id,
            client,
            producer,
        })
    }
}

impl MessageProducer {
    /// Cleans up the stream, topic, and user created by this `MessageProducer` and shuts down the underlying client.
    ///
    /// # Errors
    ///
    /// Returns an `IggyError` if the stream, topic, user deletion, or client shutdown fails.
    ///
    pub async fn clean_up_and_shutdown(&self) -> Result<(), IggyError> {
        // Connect client
        self.client.connect().await.expect("Failed to connect");

        // Create identifiers for stream, topic, and user.
        let stream_id =
            Identifier::from_str_value(self.stream_id.as_str()).expect("Invalid stream id");
        let topic_id =
            Identifier::from_str_value(self.topic_id.as_str()).expect("Invalid topic id");
        let user_id = Identifier::from_str_value(self.user_id.as_str()).expect("Invalid user id");

        // Delete the topic
        self.client
            .delete_topic(&stream_id, &topic_id)
            .await
            .expect("Failed to delete topic");

        // Delete the stream
        self.client
            .delete_stream(&stream_id)
            .await
            .expect("Failed to delete stream");

        // Delete the user
        self.client
            .delete_user(&user_id)
            .await
            .expect("Failed to delete user");

        // Shutdown
        self.client.shutdown().await.expect("Failed to shutdown");

        Ok(())
    }

    /// Shuts down the underlying client.
    ///
    /// # Errors
    ///
    /// Returns an `IggyError` if the client shutdown fails.
    ///
    pub async fn shutdown(&self) -> Result<(), IggyError> {
        // Connect client
        self.client.connect().await.expect("Failed to connect");

        // Shutdown
        self.client.shutdown().await
    }
}
