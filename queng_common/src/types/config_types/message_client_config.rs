use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageClientConfig {
    id: u16,
    name: String,
}

impl MessageClientConfig {
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }
}

impl Default for MessageClientConfig {
    fn default() -> Self {
        Self {
            id: 0,
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
