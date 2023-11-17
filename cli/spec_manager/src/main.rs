use common::prelude::HostEndpoint;
use dbgw_client::DBGatewayClient;

use crate::types::{ServiceOP, SpecType, WorkflowOP};
use crate::types::SpecType::*;

mod svc_handle;
mod types;
mod wkf_handle;

pub const SPEC_TYPE: SpecType = Workflow;
pub const SPEC_OP: ServiceOP = ServiceOP::ReadAllServices;
pub const WORKFLOW_OP: WorkflowOP = WorkflowOP::SetCheckOnline;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
