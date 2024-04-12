use std::sync::Arc;
use tokio::sync::RwLock;

pub type Guarded<T> = Arc<RwLock<T>>;
