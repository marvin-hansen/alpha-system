/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_data_bar::OHLCVBar;

pub trait PatternTrait {
    fn get_eval_result(&self, index: usize) -> Result<bool, String>;
    fn get_pattern_len(&self) -> Result<usize, String>;
    fn update_patterns(&mut self, window: &[OHLCVBar; 6]) -> Result<(), String>;
}
