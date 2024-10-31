use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaExchangesRoot {
    pub result: String,
    pub data: Vec<MetaExchange>,
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaExchange {
    pub code: String,
    pub name: String,
    pub kaiko_legacy_slug: String,
}

impl MetaExchange {
    pub fn hash(&self) -> String {
        let binding = format!("{}{}", self.code, self.name);
        let input = binding.as_bytes();
        let hash = blake3::hash(input);
        hash.to_string()
    }
}

impl Display for MetaExchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MetaExchange: {:?}", self)
    }
}
