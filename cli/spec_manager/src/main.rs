use std::error::Error;
use common::prelude::HostEndpoint;
use dbgw_client::DBGatewayClient;

use crate::types::SpecType::*;
use crate::types::{ServiceOP, SpecType, WorkflowOP};

mod svc_handle;
mod types;
mod wkf_handle;

pub const SPEC_TYPE: SpecType = ServiceConfig;
pub const SPEC_OP: ServiceOP = ServiceOP::CreateAllService;
pub const WORKFLOW_OP: WorkflowOP = WorkflowOP::SetCheckOnline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {

    let host_endpoint = HostEndpoint::new("127.0.0.1", 6060);
    let client = DBGatewayClient::new(host_endpoint).await;

    match SPEC_TYPE {
        ServiceConfig => {
            svc_handle::handle_service_op(&client, SPEC_OP)
                .await
                .expect("Failed to handle service op");
        }
        Workflow => {
            wkf_handle::handle_workflow_op(&client, WORKFLOW_OP)
                .await
                .expect("Failed to handle workflow op");
        }
    }

    Ok(())
}
