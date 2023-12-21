use crate::types::WorkflowOP;
use std::error::Error;

pub async fn handle_workflow_op(op: &WorkflowOP) -> Result<(), Box<dyn Error>> {
    match op {
        WorkflowOP::LoginLogout => {
            handle_login_logout().await.unwrap();
        }
        WorkflowOP::LoginStartData => {
            handle_login_start_data().await.unwrap();
        }
        WorkflowOP::LoginStartStopData => {
            handle_login_start_data_stop_data().await.unwrap();
        }
        WorkflowOP::LoginStartStopDataLogout => {
            handle_login_start_data_stop_data_logout().await.unwrap();
        }
    }

    Ok(())
}

async fn handle_login_logout() -> Result<(), Box<dyn Error>> {
    println!("Starting data");

    Ok(())
}

async fn handle_login_start_data() -> Result<(), Box<dyn Error>> {
    println!("Stopping data");

    Ok(())
}

async fn handle_login_start_data_stop_data() -> Result<(), Box<dyn Error>> {
    println!("Running test data");

    Ok(())
}

async fn handle_login_start_data_stop_data_logout() -> Result<(), Box<dyn Error>> {
    println!("Running test data");

    Ok(())
}
