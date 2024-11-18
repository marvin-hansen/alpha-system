use crate::MessageProducer;
use iggy::clients::producer::IggyProducer;
use iggy::identifier::Identifier;

impl MessageProducer {
    /// Returns a reference to the user identifier.
    pub fn user_id(&self) -> &Identifier {
        &self.user_id
    }

    /// Returns a reference to the stream identifier.
    pub fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    /// Returns a reference to the topic identifier.
    pub fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    /// Returns a reference to the IggyProducer.
    pub fn producer(&self) -> &IggyProducer {
        &self.producer
    }
}
