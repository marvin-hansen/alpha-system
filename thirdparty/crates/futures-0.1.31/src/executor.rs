//! Executors
//!
//! This module contains tools for managing the raw execution of futures,
//! which is needed when building *executors* (places where futures can run).
//!
//! More information about executors can be [found online at tokio.rs][online].
//!
//! [online]: https://tokio.rs/docs/going-deeper-futures/tasks/

#[allow(deprecated)]
#[doc(hidden)]
#[cfg(feature = "use_std")]
pub use task_impl::{Executor, Run, Unpark};

pub use task_impl::{spawn, with_notify, Notify, Spawn};

pub use task_impl::{NotifyHandle, UnsafeNotify};
