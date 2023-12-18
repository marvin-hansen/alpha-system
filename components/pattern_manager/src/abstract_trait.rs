use common::prelude::DataBar;

pub trait PatternTrait {
    fn get_eval_result(&self, index: usize) -> Result<bool, String>;
    fn get_pattern_len(&self) -> Result<usize, String>;
    fn update_patterns(&mut self, window: &[DataBar; 6]) -> Result<(), String>;
}
