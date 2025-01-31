/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

pub(crate) mod print_utils;
mod types;
pub(crate) mod workflow;

pub(crate) const DBG: bool = false;

pub use crate::types::worflow_op::*;
pub use crate::workflow::determine_workflow;
pub use crate::workflow::execute_workflow;
