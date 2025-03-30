/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

mod blocking_wait_strategy;
mod spinlock_wait_strategy;

// Re exports
pub use blocking_wait_strategy::*;
pub use spinlock_wait_strategy::*;
