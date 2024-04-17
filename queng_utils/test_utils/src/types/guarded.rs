use std::sync::Arc;
use std::sync::RwLock;
pub type Guarded<T> = Arc<RwLock<T>>;
