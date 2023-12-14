use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum WorkflowOP {
    #[default]
    StartData,
    StopData,
}

impl Display for WorkflowOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowOP::StartData => write!(f, "StartData"),
            WorkflowOP::StopData => write!(f, "StopData"),
        }
    }
}