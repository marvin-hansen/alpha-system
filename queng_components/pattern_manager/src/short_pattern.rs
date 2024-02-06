/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use crate::abstract_trait::PatternTrait;
use crate::fields::{ONE_HUNDRED, POINT_FIVE};
use common::prelude::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

const SIZE: usize = 9;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShortPattern {
    arr: [bool; SIZE],
}

impl ShortPattern {
    pub fn new() -> Self {
        Self { arr: [false; SIZE] }
    }
}

impl PatternTrait for ShortPattern {
    fn get_eval_result(&self, index: usize) -> Result<bool, String> {
        if index >= self.arr.len() {
            return Err(format!("short_pattern: index out of bound: {}", index));
        }

        Ok(self.arr[index])
    }

    fn get_pattern_len(&self) -> Result<usize, String> {
        Ok(SIZE)
    }

    fn update_patterns(&mut self, window: &[OHLCVBar; 6]) -> Result<(), String> {
        let last_idx = window.len() - 1;
        let day_0_bar = &window[last_idx];
        let day_1_bar = &window[last_idx - 1];
        let day_2_bar = &window[last_idx - 2];
        let day_3_bar = &window[last_idx - 3];
        // let day_4_bar = &window[last_idx - 4];
        let day_5_bar = &window[last_idx - 5];

        // let opend0 = day_0_bar.open().to_f64().unwrap();
        // let highd0 = day_0_bar.high().to_f64().unwrap();
        let lowd0 = day_0_bar.low().to_f64().unwrap();
        // let closed0 = day_0_bar.close().to_f64().unwrap();
        // let opend1 = day_1_bar.open().to_f64().unwrap();
        let highd1 = day_1_bar.high().to_f64().unwrap();
        let lowd1 = day_1_bar.low().to_f64().unwrap();
        let closed1 = day_1_bar.close().to_f64().unwrap();
        let highd2 = day_2_bar.high().to_f64().unwrap();
        let lowd2 = day_2_bar.low().to_f64().unwrap();
        let closed2 = day_2_bar.close().to_f64().unwrap();
        let highd3 = day_3_bar.high().to_f64().unwrap();
        // let lowd3 = day_3_bar.low().to_f64().unwrap();
        let closed3 = day_3_bar.close().to_f64().unwrap();
        // let highd4 = day_4_bar.high().to_f64().unwrap();
        // let lowd4 = day_4_bar.low().to_f64().unwrap();
        // let closed4 = day_4_bar.close().to_f64().unwrap();
        // let opend5 = day_5_bar.open().to_f64().unwrap();
        // let highd5 = day_5_bar.high().to_f64().unwrap();
        let lowd5 = day_5_bar.low().to_f64().unwrap();
        // let closed5 = day_5_bar.close().to_f64().unwrap();

        self.arr[0] = false;

        // 11 in BasePattern
        self.arr[1] = (highd1 < highd2) && (lowd1 < lowd2);

        // Custom pattern
        self.arr[2] = (highd1 < highd3) && (lowd1 < lowd2);

        // 15 in BasePattern
        self.arr[3] = closed1 < closed2;

        // Custom pattern
        self.arr[4] = closed1 < closed3;

        // 18 in BasePattern
        self.arr[5] = closed1 < (closed2 - closed2 * POINT_FIVE / ONE_HUNDRED);

        // 22 in BasePattern
        self.arr[6] = lowd0 < lowd1;

        // 23 in BasePattern
        self.arr[7] = lowd0 < lowd5;

        // Custom pattern
        self.arr[8] = lowd0 < lowd1 && lowd0 < lowd2;

        Ok(())
    }
}
