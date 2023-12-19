use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum SpecType {
    #[default]
    ServiceConfig,
    Workflow,
}

impl Display for SpecType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecType::ServiceConfig => write!(f, "ServiceConfig"),
            SpecType::Workflow => write!(f, "Workflow"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum WorkflowOP {
    #[default]
    CreateRead,
    SetCheckOnline,
}

impl Display for WorkflowOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowOP::CreateRead => write!(f, "CreateRead"),
            WorkflowOP::SetCheckOnline => write!(f, "SetCheckOnline"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ServiceOP {
    CreateAllService,
    CreateService,
    CheckIfServiceIDExists,
    CheckIfServicesExists,
    CheckServiceIDOnline,
    CheckServicesOnline,
    SetServiceOnline,
    SetServiceOffline,
    ReadAllServices,
    ReadServiceById,
    UpdateService,
    DeleteService,
    DeleteAllServices,
}

impl Display for ServiceOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceOP::CreateAllService => write!(f, "CreateAllService"),
            ServiceOP::CreateService => write!(f, "CreateService"),
            ServiceOP::CheckIfServiceIDExists => write!(f, "CheckIfServiceIDExists"),
            ServiceOP::CheckIfServicesExists => write!(f, "CheckIfServicesExists"),
            ServiceOP::CheckServiceIDOnline => write!(f, "CheckServiceIDOnline"),
            ServiceOP::CheckServicesOnline => write!(f, "CheckServicesOnline"),
            ServiceOP::SetServiceOnline => write!(f, "SetServiceOnline"),
            ServiceOP::SetServiceOffline => write!(f, "SetServiceOffline"),
            ServiceOP::ReadAllServices => write!(f, "ReadAllServices"),
            ServiceOP::ReadServiceById => write!(f, "ReadServiceById"),
            ServiceOP::UpdateService => write!(f, "UpdateService"),
            ServiceOP::DeleteService => write!(f, "DeleteService"),
            ServiceOP::DeleteAllServices => write!(f, "DeleteAllServices"),
        }
    }
}
