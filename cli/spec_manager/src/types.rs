use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum SpecType {
    #[default]
    ServiceConfig,
}

impl Display for SpecType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecType::ServiceConfig => write!(f, "ServiceConfig"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ServiceOP {
    CreateAllService,
    CreateService,
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
            ServiceOP::ReadAllServices => write!(f, "ReadAllServices"),
            ServiceOP::ReadServiceById => write!(f, "ReadRecordById"),
            ServiceOP::UpdateService => write!(f, "UpdateService"),
            ServiceOP::DeleteService => write!(f, "DeleteService"),
            ServiceOP::DeleteAllServices => write!(f, "DeleteAllServices"),
        }
    }
}
