use crate::prelude::PatternType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct PatternConfig {
    pattern_config_id: u32,
    pattern_config_name: String,
    pattern_config_description: String,
    pattern_type: PatternType,
    pattern_long_yes: u8,
    pattern_long_no: u8,
    pattern_short_yes: u8,
    pattern_short_no: u8,
}

impl PatternConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pattern_config_id: u32,
        pattern_config_name: String,
        pattern_config_description: String,
        pattern_type: PatternType,
        pattern_long_yes: u8,
        pattern_long_no: u8,
        pattern_short_yes: u8,
        pattern_short_no: u8,
    ) -> Self {
        Self {
            pattern_config_id,
            pattern_config_name,
            pattern_config_description,
            pattern_type,
            pattern_long_yes,
            pattern_long_no,
            pattern_short_yes,
            pattern_short_no,
        }
    }
}

impl PatternConfig {
    pub fn pattern_config_id(&self) -> u32 {
        self.pattern_config_id
    }
    pub fn pattern_config_name(&self) -> &str {
        &self.pattern_config_name
    }
    pub fn pattern_config_description(&self) -> &str {
        &self.pattern_config_description
    }
    pub fn pattern_type(&self) -> &PatternType {
        &self.pattern_type
    }
    pub fn pattern_long_yes(&self) -> u8 {
        self.pattern_long_yes
    }
    pub fn pattern_long_no(&self) -> u8 {
        self.pattern_long_no
    }
    pub fn pattern_short_yes(&self) -> u8 {
        self.pattern_short_yes
    }
    pub fn pattern_short_no(&self) -> u8 {
        self.pattern_short_no
    }
}

impl Display for PatternConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PatternConfig {{ pattern_config_id: {}, pattern_config_name: \"{}\",\
             pattern_config_description: \"{}\", pattern_type: {:?}, pattern_long_yes: {}, \
             pattern_long_no: {}, pattern_short_yes: {}, pattern_short_no: {} }}",
            self.pattern_config_id,
            self.pattern_config_name,
            self.pattern_config_description,
            self.pattern_type,
            self.pattern_long_yes,
            self.pattern_long_no,
            self.pattern_short_yes,
            self.pattern_short_no
        )
    }
}
