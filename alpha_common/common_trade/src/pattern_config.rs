/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use std::fmt::{Display, Formatter};

use crate::PatternType;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    #[must_use]
    pub const fn new(
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
    #[must_use]
    pub const fn pattern_config_id(&self) -> u32 {
        self.pattern_config_id
    }
    #[must_use]
    pub fn pattern_config_name(&self) -> &str {
        &self.pattern_config_name
    }
    #[must_use]
    pub fn pattern_config_description(&self) -> &str {
        &self.pattern_config_description
    }
    #[must_use]
    pub const fn pattern_type(&self) -> &PatternType {
        &self.pattern_type
    }
    #[must_use]
    pub const fn pattern_long_yes(&self) -> u8 {
        self.pattern_long_yes
    }
    #[must_use]
    pub const fn pattern_long_no(&self) -> u8 {
        self.pattern_long_no
    }
    #[must_use]
    pub const fn pattern_short_yes(&self) -> u8 {
        self.pattern_short_yes
    }
    #[must_use]
    pub const fn pattern_short_no(&self) -> u8 {
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
