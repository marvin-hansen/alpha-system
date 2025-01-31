/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::EventConsumerError;

#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EventConsumer: Send)]
pub trait LocalEventConsumer {
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError>;
}

impl<T: EventConsumer + Send + Sync> EventConsumer for &T {
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError> {
        (**self).consume(data).await
    }
}
