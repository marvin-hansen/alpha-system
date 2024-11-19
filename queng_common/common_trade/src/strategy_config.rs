use std::fmt::{Display, Formatter};

use crate::{PatternConfig, TradeEntryType, TradeStrategyType};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct StrategyConfig {
    strategy_id: String,
    strategy_name: String,
    strategy_description: String,
    // See strategyType enum for all available choices
    strategy_type: TradeStrategyType,
    // Closes position at end of trading day. Yes / No
    intraday: bool,
    // Day NOT to trade. Monday is 1, Friday is 5, and Sunday is 7;
    day_to_filter: u8,
    // Sets whether to enter at current or next bar.
    trade_entry_type: TradeEntryType,
    // Turbo strategies only, set to None for other strategies.
    pattern_config: Option<PatternConfig>,
    // PT/SL = Actual dollar  PnL value per one instrument
    profit_target: u32,
    // i.e. $150 for one contract or one Lot
    stop_target: u32,
    // Nr. of bars to auto-close position in case pt/ls were not triggered before.
    max_nr_bars: u32,
}

impl StrategyConfig {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        strategy_id: String,
        strategy_name: String,
        strategy_description: String,
        strategy_type: TradeStrategyType,
        intraday: bool,
        day_to_filter: u8,
        trade_entry_type: TradeEntryType,
        pattern_config: Option<PatternConfig>,
        profit_target: u32,
        stop_target: u32,
        max_nr_bars: u32,
    ) -> Self {
        Self {
            strategy_id,
            strategy_name,
            strategy_description,
            strategy_type,
            intraday,
            day_to_filter,
            trade_entry_type,
            pattern_config,
            profit_target,
            stop_target,
            max_nr_bars,
        }
    }
}

impl StrategyConfig {
    #[must_use]
    pub fn strategy_id(&self) -> &str {
        &self.strategy_id
    }
    #[must_use]
    pub fn strategy_name(&self) -> &str {
        &self.strategy_name
    }
    #[must_use]
    pub fn strategy_description(&self) -> &str {
        &self.strategy_description
    }
    #[must_use]
    pub const fn strategy_type(&self) -> &TradeStrategyType {
        &self.strategy_type
    }
    #[must_use]
    pub const fn intraday(&self) -> bool {
        self.intraday
    }
    #[must_use]
    pub const fn day_to_filter(&self) -> u8 {
        self.day_to_filter
    }
    #[must_use]
    pub const fn trade_entry_type(&self) -> &TradeEntryType {
        &self.trade_entry_type
    }
    #[must_use]
    pub const fn pattern_config(&self) -> &Option<PatternConfig> {
        &self.pattern_config
    }
    #[must_use]
    pub const fn profit_target(&self) -> u32 {
        self.profit_target
    }
    #[must_use]
    pub const fn stop_target(&self) -> u32 {
        self.stop_target
    }
    #[must_use]
    pub const fn max_nr_bars(&self) -> u32 {
        self.max_nr_bars
    }
}

impl Display for StrategyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StrategyConfig {{ strategy_id: {}, strategy_name: {}, strategy_description: {}, strategy_type: {:?}, intraday: {}, day_to_filter: {}, trade_entry_type: {:?}, pattern_config: {:?}, profit_target: {}, stop_target: {}, max_nr_bars: {} }}",
               self.strategy_id,
               self.strategy_name,
               self.strategy_description,
               self.strategy_type,
               self.intraday,
               self.day_to_filter,
               self.trade_entry_type,
               self.pattern_config,
               self.profit_target,
               self.stop_target,
               self.max_nr_bars
        )
    }
}
