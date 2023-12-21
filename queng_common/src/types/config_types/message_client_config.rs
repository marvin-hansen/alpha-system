use crate::prelude::ServiceID;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageClientConfig {
    id: u16,
    name: String,
}

impl MessageClientConfig {
    pub fn new(id: u16, name: String) -> Self {
        // Prevents ID clash with configurations generated from ServiceID ENUM
        assert!(id > 20, "id must be greater than 14");

        // SBE Login & Logout message defines name as non-empty 10 characters
        assert_eq!(name.len(), 10, "name must be exactly 10 characters long");

        Self { id, name }
    }

    pub fn from_svc_id(svc_id: ServiceID) -> Self {
        let id = svc_id.id().into();
        let name = svc_id.name();

        // Prevents ID clash with manually created configurations
        assert!(id < 20, "id must be less than 14");
        // SBE Login & Logout message defines name as non-empty 10 characters
        assert!(name.len() > 0, "name cannot be empty");
        assert!(name.len() < 11, "name must be at most 10 characters long");

        Self { id, name }
    }
}

impl Default for MessageClientConfig {
    fn default() -> Self {
        Self {
            id: 100,
            name: String::from("default_client"),
        }
    }
}

impl MessageClientConfig {
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.name, "control")
    }

    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.name, "data")
    }

    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.name, "execution")
    }
}

impl MessageClientConfig {
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for MessageClientConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "MessageClientConfig {{ id: {}, name: {} }}",
            self.id, self.name
        )
    }
}
