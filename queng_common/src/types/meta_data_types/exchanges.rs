use serde::{Deserialize, Serialize};

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
