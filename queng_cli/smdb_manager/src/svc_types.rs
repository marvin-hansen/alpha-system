use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ServiceOP {
    // All,
    CheckIfServiceIDExists,
    CheckIfAllServicesExists,
    CheckServiceIDOnline,
    CheckAllServicesOnline,
    SetServiceOnline,
    SetServiceOffline,
}

impl Display for ServiceOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // ServiceOP::All => write!(f, "All"),
            ServiceOP::CheckIfServiceIDExists => write!(f, "CheckIfServiceIDExists"),
            ServiceOP::CheckIfAllServicesExists => write!(f, "CheckIfServicesExists"),
            ServiceOP::CheckServiceIDOnline => write!(f, "CheckServiceIDOnline"),
            ServiceOP::CheckAllServicesOnline => write!(f, "CheckServicesOnline"),
            ServiceOP::SetServiceOnline => write!(f, "SetServiceOnline"),
            ServiceOP::SetServiceOffline => write!(f, "SetServiceOffline"),
        }
    }
}
