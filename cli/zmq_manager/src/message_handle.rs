use std::error::Error;
use crate::types::WorkflowOP;

pub async fn handle_workflow_op(op: &WorkflowOP) -> Result<(), Box<dyn Error>>  {

    match op {

        WorkflowOP::StartData => {
            handle_start_data().await.expect("Failed to handle start data");
        }
        WorkflowOP::StopData => {
            handle_stop_data().await.expect("Failed to handle stop data");
        }
    }

    Ok(())
}

async fn handle_start_data() -> Result<(), Box<dyn Error>>  {
    println!("Starting data");

    Ok(())
}

async fn handle_stop_data() -> Result<(), Box<dyn Error>>  {
    println!("Stopping data");

    Ok(())
}