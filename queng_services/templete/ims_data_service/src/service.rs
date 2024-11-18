type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct Server {}

impl Server {
    pub async fn new() -> Self {
        Self::build().await
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

impl Server {
    pub async fn build() -> Self {
        Self {}
    }
}
