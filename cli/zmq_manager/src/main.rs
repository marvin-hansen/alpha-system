mod types;
mod message_handle;

use std::error::Error;
use crate::message_handle::handle_workflow_op;
use crate::types::WorkflowOP;

const OP: WorkflowOP = WorkflowOP::StopData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    handle_workflow_op(&OP).await.expect("Failed to handle workflow op");

    Ok(())
}
