mod event_processor;
mod getters;

use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use message_shared::Args;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct MessageProducer {
    dbg: bool,
    stream_id: Identifier,
    topic_id: Identifier,
    producer: Arc<IggyProducer>,
}

impl MessageProducer {
    /// Creates a new `MessageProducer` instance using the provided `IggyClient` and identifiers.
    ///
    /// # Arguments
    ///
    /// * `client` - The `IggyClient` to use for authentication and communication.
    /// * `stream_id` - The identifier of the stream.
    /// * `topic_id` - The identifier of the topic.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `MessageProducer` instance or an `IggyError`.
    ///
    pub async fn from_client(
        dbg: bool,
        client: &IggyClient,
        stream_id: String,
        topic_id: String,
    ) -> Result<Self, IggyError> {
        let args = Args::new(stream_id, topic_id);
        Self::build(dbg, args, client).await
    }
}

impl MessageProducer {
    async fn build(dbg: bool, args: Args, client: &IggyClient) -> Result<Self, IggyError> {
        let dbg_print = |msg: &str| {
            if dbg {
                println!("[MessageProducer]: {msg}");
            }
        };

        dbg_print("Create Identifiers for control stream and topic");
        let stream_id = Identifier::from_str_value(&args.stream_id)
            .expect("[MessageProducer]: Invalid stream id");

        let topic_id = Identifier::from_str_value(&args.topic_id)
            .expect("[MessageProducer]: Invalid topic id");

        dbg_print("Creating producer");
        let mut producer = client
            .producer(&args.stream_id, &args.topic_id)
            .expect("[MessageProducer]: Failed to create producer")
            .batch_size(args.messages_per_batch)
            .send_interval(
                IggyDuration::from_str(&args.interval)
                    .expect("[MessageProducer]: Invalid interval format"),
            )
            .partitioning(Partitioning::balanced())
            .build();

        dbg_print("Initializing producer");
        //
        // @FIXME:
        // [MessageProducer]: Failed to init producer: StreamNameAlreadyExists("")
        //
        producer
            .init()
            .await
            .expect("[MessageProducer]: Failed to init producer");

        Ok(Self {
            dbg,
            stream_id,
            topic_id,
            producer: Arc::new(producer),
        })
    }
}

impl MessageProducer {
    pub(crate) fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[MessageProducer]: {msg}");
        }
    }
}
