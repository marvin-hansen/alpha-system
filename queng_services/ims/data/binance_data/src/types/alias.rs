use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type Guarded<T> = Arc<RwLock<T>>;
