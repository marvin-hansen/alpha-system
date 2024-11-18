mod abstract_trait;
mod base_pattern;
mod extra_pattern;
mod fields;
mod long_pattern;
mod short_pattern;

use crate::base_pattern::BasePattern;
use crate::extra_pattern::ExtraPattern;
use crate::long_pattern::LongPattern;
use crate::short_pattern::ShortPattern;
use abstract_trait::PatternTrait;
use common_data_bar::OHLCVBar;
use common_trade::prelude::PatternType;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct PatternManager {
    base_pattern: RefCell<BasePattern>,
    extra_pattern: RefCell<ExtraPattern>,
    long_pattern: RefCell<LongPattern>,
    short_pattern: RefCell<ShortPattern>,
}

impl Default for PatternManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternManager {
    pub fn new() -> Self {
        Self {
            base_pattern: RefCell::new(BasePattern::new()),
            extra_pattern: RefCell::new(ExtraPattern::new()),
            long_pattern: RefCell::new(LongPattern::new()),
            short_pattern: RefCell::new(ShortPattern::new()),
        }
    }
}

impl PatternManager {
    pub fn get_eval_result(
        &self,
        pattern_type: &PatternType,
        index: usize,
    ) -> Result<bool, String> {
        match pattern_type {
            PatternType::Base => self.base_pattern.borrow().get_eval_result(index),
            PatternType::Extra => self.extra_pattern.borrow().get_eval_result(index),
            PatternType::Long => self.long_pattern.borrow().get_eval_result(index),
            PatternType::Short => self.short_pattern.borrow().get_eval_result(index),

            PatternType::NullVal => Err("Error: Null pattern type".to_string()),
        }
    }

    pub fn get_pattern_len(&self, pattern_type: &PatternType) -> Result<usize, String> {
        match pattern_type {
            PatternType::Base => self.base_pattern.borrow().get_pattern_len(),
            PatternType::Extra => self.extra_pattern.borrow().get_pattern_len(),
            PatternType::Long => self.long_pattern.borrow().get_pattern_len(),
            PatternType::Short => self.short_pattern.borrow().get_pattern_len(),
            PatternType::NullVal => Err("Error: Null pattern type".to_string()),
        }
    }

    pub fn update_patterns(
        &self,
        pattern_type: &PatternType,
        window: &[OHLCVBar; 6],
    ) -> Result<(), String> {
        match pattern_type {
            PatternType::Base => self.base_pattern.borrow_mut().update_patterns(window),
            PatternType::Extra => self.extra_pattern.borrow_mut().update_patterns(window),
            PatternType::Long => self.long_pattern.borrow_mut().update_patterns(window),
            PatternType::Short => self.short_pattern.borrow_mut().update_patterns(window),
            PatternType::NullVal => Err("Error: Null pattern type".to_string()),
        }
    }
}
