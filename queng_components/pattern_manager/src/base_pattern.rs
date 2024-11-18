use crate::abstract_trait::PatternTrait;
use crate::fields::{
    ONE, ONE_HUNDRED, ONE_POINT_FIVE, POINT_FIVE, POINT_SEVEN_FIVE, POINT_TWO, THREE,
};
use common_data_bar::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

const SIZE: usize = 43;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BasePattern {
    arr: [bool; SIZE],
}

impl BasePattern {
    pub fn new() -> Self {
        Self { arr: [false; SIZE] }
    }
}

impl PatternTrait for BasePattern {
    fn get_eval_result(&self, index: usize) -> Result<bool, String> {
        if index >= self.arr.len() {
            return Err(format!("base_pattern: index out of bound: {}", index));
        }

        Ok(self.arr[index])
    }
    fn get_pattern_len(&self) -> Result<usize, String> {
        Ok(SIZE)
    }
    fn update_patterns(&mut self, window: &[OHLCVBar; 6]) -> Result<(), String> {
        let last_idx = window.len() - 1;
        let day_0_bar = &window.clone()[last_idx];
        let day_1_bar = &window[last_idx - 1];
        let day_2_bar = &window[last_idx - 2];
        let day_3_bar = &window[last_idx - 3];
        let day_4_bar = &window[last_idx - 4];
        let day_5_bar = &window[last_idx - 5];

        let opend0 = day_0_bar.open().to_f64().unwrap();
        let highd0 = day_0_bar.high().to_f64().unwrap();
        let lowd0 = day_0_bar.low().to_f64().unwrap();
        let opend1 = day_1_bar.open().to_f64().unwrap();
        let highd1 = day_1_bar.high().to_f64().unwrap();
        let lowd1 = day_1_bar.low().to_f64().unwrap();
        let closed1 = day_1_bar.close().to_f64().unwrap();
        let highd2 = day_2_bar.high().to_f64().unwrap();
        let lowd2 = day_2_bar.low().to_f64().unwrap();
        let closed2 = day_2_bar.close().to_f64().unwrap();
        let highd3 = day_3_bar.high().to_f64().unwrap();
        let lowd3 = day_3_bar.low().to_f64().unwrap();
        let closed3 = day_3_bar.close().to_f64().unwrap();
        let highd4 = day_4_bar.high().to_f64().unwrap();
        let lowd4 = day_4_bar.low().to_f64().unwrap();
        let closed4 = day_4_bar.close().to_f64().unwrap();
        let opend5 = day_5_bar.open().to_f64().unwrap();
        let highd5 = day_5_bar.high().to_f64().unwrap();
        let lowd5 = day_5_bar.low().to_f64().unwrap();
        let closed5 = day_5_bar.close().to_f64().unwrap();

        self.arr[0] = false;
        self.arr[1] = math_utils::abs(opend1 - closed1) < ((highd1 - lowd1) * POINT_FIVE);
        self.arr[2] = math_utils::abs(opend1 - closed5) < ((highd5 - closed1) * POINT_FIVE);
        self.arr[3] = math_utils::abs(opend5 - closed1)
            < (math_utils::max(&[highd1, highd2, highd3, highd4, highd5])
                - (math_utils::min(&[lowd1, lowd2, lowd3, lowd4, lowd5])) * POINT_FIVE);
        self.arr[4] = (highd0 - opend0) > ((highd1 - opend1) * ONE);
        self.arr[5] = (highd0 - opend0) > ((highd1 - opend1) * ONE_POINT_FIVE);
        self.arr[6] = (opend0 - lowd0) > ((opend1 - lowd1) * ONE);
        self.arr[7] = (opend0 - lowd0) > ((opend1 - lowd1) * ONE_POINT_FIVE);
        self.arr[8] = (closed1 > closed2) && (closed2 > closed3) && (closed3 > closed4);
        self.arr[9] = (closed1 < closed2) && (closed2 < closed3) && (closed3 < closed4);
        self.arr[10] = (highd1 > highd2) && (lowd1 > lowd2);

        self.arr[11] = (highd1 < highd2) && (lowd1 < lowd2);
        self.arr[12] = highd0 > (lowd0 + lowd0 * POINT_SEVEN_FIVE / ONE_HUNDRED);
        self.arr[13] = highd0 < (lowd0 + lowd0 * POINT_SEVEN_FIVE / ONE_HUNDRED);
        self.arr[14] = closed1 > closed2;
        self.arr[15] = closed1 < closed2;
        self.arr[16] = closed1 < opend1;
        self.arr[17] = closed1 > opend1;
        self.arr[18] = closed1 < (closed2 - closed2 * POINT_FIVE / ONE_HUNDRED);
        self.arr[19] = closed1 > (closed2 + closed2 * POINT_FIVE / ONE_HUNDRED);
        self.arr[20] = highd0 > highd1;

        self.arr[21] = highd1 > highd5;
        self.arr[22] = lowd0 < lowd1;
        self.arr[23] = lowd1 < lowd5;
        self.arr[24] = (highd1 > highd2) && (highd1 > highd3) && (highd1 > highd4);
        self.arr[25] = (highd1 < highd2) && (highd1 < highd3) && (highd1 < highd4);
        self.arr[26] = (lowd1 < lowd2) && (lowd1 < lowd3) && (lowd1 < lowd4);
        self.arr[27] = (lowd1 > lowd2) && (lowd1 > lowd3) && (lowd1 > lowd4);
        self.arr[28] = (closed1 > closed2) && (closed2 > closed3) && (opend0 > closed1);
        self.arr[29] = (closed1 < closed2) && (closed2 < closed3) && (opend0 < closed1);
        self.arr[30] = (highd1 - closed1) < ((highd1 - lowd1) * POINT_TWO);

        self.arr[31] = (closed1 - lowd1) < ((highd1 - lowd1) * POINT_TWO);
        self.arr[32] = opend0 < lowd1 || opend0 > highd1;
        self.arr[33] = opend0 < (closed1 - closed1 * POINT_FIVE / ONE_HUNDRED);
        self.arr[34] = opend0 > (closed1 + closed1 * POINT_FIVE / ONE_HUNDRED);
        self.arr[35] = highd0 < highd1 && lowd0 > lowd1;
        self.arr[36] = (highd1 - lowd1) < (((highd2 - lowd2) + (highd3 - lowd3)) / THREE);
        self.arr[37] = (highd1 - lowd1) < (highd2 - lowd2) && (highd2 - lowd2) < (highd3 - lowd3);
        self.arr[38] = (highd1 - lowd1) < (highd2 - lowd2) && (highd2 - lowd2) < (highd3 - lowd3);
        self.arr[39] = highd1 < highd2 || lowd1 > lowd2;
        self.arr[40] = highd2 < highd1 && lowd2 > lowd1;

        self.arr[41] = true;
        self.arr[42] = false;

        Ok(())
    }
}
