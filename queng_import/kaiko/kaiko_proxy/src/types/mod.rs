use crate::types::meta_data_set::MetaDataSet;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub(crate) mod health;
pub(crate) mod meta_data_set;

pub(crate) type MetaDataStore = Arc<ArcSwap<MetaDataSet>>;
