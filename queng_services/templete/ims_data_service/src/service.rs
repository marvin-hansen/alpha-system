type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct Server {}

impl Server {
    pub async fn new() -> Self {
        Self::build().await
    }
}

impl Server {
    pub async fn build() -> Self {
        Self {}
    }
}
