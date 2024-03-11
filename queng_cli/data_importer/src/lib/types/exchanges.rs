use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub result: String,
    pub data: Vec<Exchange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub code: String,
    pub name: String,
    #[serde(rename = "kaiko_legacy_slug")]
    pub kaiko_legacy_slug: String,
}
