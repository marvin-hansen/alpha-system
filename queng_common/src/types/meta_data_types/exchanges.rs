use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangesRoot {
    pub result: String,
    pub data: Vec<Exchange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    pub code: String,
    pub name: String,
    pub kaiko_legacy_slug: String,
}
