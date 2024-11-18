/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use crate::abstract_trait::PatternTrait;
use crate::fields::*;
use common_data_bar::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

const SIZE: usize = 139;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtraPattern {
    arr: [bool; SIZE],
}

impl ExtraPattern {
    pub fn new() -> Self {
        ExtraPattern { arr: [false; SIZE] }
    }
}

impl PatternTrait for ExtraPattern {
    fn get_eval_result(&self, index: usize) -> Result<bool, String> {
        if index >= self.arr.len() {
            return Err(format!("extra_pattern: index out of bound: {}", index));
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
        let day_4_bar = &window[last_idx - 4];
        let day_5_bar = &window[last_idx - 5];

        let opend0 = day_0_bar.open().to_f64().unwrap();
        let highd0 = day_0_bar.high().to_f64().unwrap();
        let lowd0 = day_0_bar.low().to_f64().unwrap();
        let opend1 = day_1_bar.open().to_f64().unwrap();
        let highd1 = day_1_bar.high().to_f64().unwrap();
        let lowd1 = day_1_bar.low().to_f64().unwrap();
        let closed1 = day_1_bar.close().to_f64().unwrap();
        let opend2 = day_2_bar.open().to_f64().unwrap();
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

        let body1d = math_utils::abs(opend1 - closed1);
        let body5d = math_utils::abs(opend5 - closed1);
        let range1d = highd1 - lowd1;
        let range5d = math_utils::max(&[highd1, highd2, highd3, highd4, highd5])
            - math_utils::min(&[lowd1, lowd2, lowd3, lowd4, lowd5]);

        self.arr[0] = false;
        self.arr[1] = body1d < POINT_ONE * range1d;
        self.arr[2] = body1d < POINT_TWO_FIVE * range1d;
        self.arr[3] = body1d < POINT_FIVE * range1d;
        self.arr[4] = body1d < POINT_SEVEN_FIVE * range1d;
        self.arr[5] = body1d > POINT_TWO_FIVE * range1d;
        self.arr[6] = body1d > POINT_FIVE * range1d;
        self.arr[7] = body1d > POINT_SEVEN_FIVE * range1d;
        self.arr[8] = body1d > POINT_NINE * range1d;
        self.arr[9] = body5d < POINT_ONE * (highd5 - lowd1);
        self.arr[10] = body5d < POINT_TWO_FIVE * (highd5 - lowd1);

        self.arr[11] = body5d < POINT_FIVE * (highd5 - lowd1);
        self.arr[12] = body5d < POINT_SEVEN_FIVE * (highd5 - lowd1);
        self.arr[13] = body5d < ONE * (highd5 - lowd1);
        self.arr[14] = body5d < ONE_POINT_FIVE * (highd5 - lowd1);
        self.arr[15] = body5d < TWO * (highd5 - lowd1);
        self.arr[16] = body5d > POINT_TWO_FIVE * (highd5 - lowd1);
        self.arr[17] = body5d > POINT_FIVE * (highd5 - lowd1);
        self.arr[18] = body5d > POINT_SEVEN_FIVE * (highd5 - lowd1);
        self.arr[19] = body5d > ONE * (highd5 - lowd1);
        self.arr[20] = body5d > ONE_POINT_FIVE * (highd5 - lowd1);

        self.arr[21] = body5d > THREE * (highd5 - lowd1);
        self.arr[22] = body5d > TWO_POINT_FIVE * (highd5 - lowd1);
        self.arr[23] = body5d < POINT_ONE * range5d;
        self.arr[24] = body5d < POINT_TWO_FIVE * range5d;
        self.arr[25] = body5d < POINT_FIVE * range5d;
        self.arr[26] = body5d < POINT_SEVEN_FIVE * range5d;
        self.arr[27] = body5d > POINT_NINE * range5d;
        self.arr[28] = body5d > POINT_TWO_FIVE * range5d;
        self.arr[29] = body5d > POINT_FIVE * range5d;
        self.arr[30] = body5d > POINT_SEVEN_FIVE * range5d;

        self.arr[31] = (highd0 - opend0) > ((highd1 - opend1) * POINT_TWO_FIVE);
        self.arr[32] = (highd0 - opend0) > ((highd1 - opend1) * POINT_FIVE);
        self.arr[33] = (highd0 - opend0) > ((highd1 - opend1) * POINT_SEVEN_FIVE);
        self.arr[34] = (highd0 - opend0) > ((highd1 - opend1) * ONE);
        self.arr[35] = (highd0 - opend0) > ((highd1 - opend1) * ONE_POINT_FIVE);
        self.arr[36] = (highd0 - opend0) > ((highd1 - opend1) * TWO);
        self.arr[37] = (highd0 - opend0) > ((highd1 - opend1) * TWO_POINT_FIVE);
        self.arr[38] = (highd0 - opend0) > ((highd1 - opend1) * THREE);
        self.arr[39] = (highd0 - opend0) < (highd1 - opend1);
        self.arr[40] = (opend0 - lowd0) < (opend1 - lowd1);

        self.arr[41] = (opend0 - lowd0) > ((opend1 - lowd1) * POINT_FIVE);
        self.arr[42] = (opend0 - lowd0) > ((opend1 - lowd1) * ONE);
        self.arr[43] = (opend0 - lowd0) > ((opend1 - lowd1) * ONE_POINT_FIVE);
        self.arr[44] = (opend0 - lowd0) > ((opend1 - lowd1) * TWO);
        self.arr[45] = (opend0 - lowd0) > ((opend1 - lowd1) * TWO_POINT_FIVE);
        self.arr[46] = (opend0 - lowd0) > ((opend1 - lowd1) * THREE);
        self.arr[47] = (closed1 > closed2) && (closed2 > closed3) && (closed3 > closed4);
        self.arr[48] = (closed1 > closed2)
            && (closed2 > closed3)
            && (closed3 > closed4)
            && (closed4 > closed5);
        self.arr[49] = (closed1 < closed2)
            && (closed2 < closed3)
            && (closed3 < closed4)
            && (closed4 < closed5);
        self.arr[50] = (highd1 > highd2) && (lowd1 > lowd2);

        self.arr[51] = (highd1 < highd2) && (lowd1 < lowd2);
        self.arr[52] = highd0 > (lowd0 + lowd0 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[53] = highd0 > (lowd0 + lowd0 * POINT_SEVEN_FIVE * POINT_ZERO_ONE);
        self.arr[54] = highd0 > (lowd0 + lowd0 * ONE * POINT_ZERO_ONE);
        self.arr[55] = highd0 > (lowd0 + lowd0 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[56] = highd0 > (lowd0 + lowd0 * TWO * POINT_ZERO_ONE);
        self.arr[57] = highd0 > (lowd0 + lowd0 * TWO_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[58] = highd0 > (lowd0 + lowd0 * THREE * POINT_ZERO_ONE);
        self.arr[59] = highd0 < (lowd0 + lowd0 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[60] = highd0 < (lowd0 + lowd0 * POINT_SEVEN_FIVE * POINT_ZERO_ONE);

        self.arr[61] = highd0 < (lowd0 + lowd0 * ONE * POINT_ZERO_ONE);
        self.arr[62] = highd0 < (lowd0 + lowd0 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[63] = highd0 < (lowd0 + lowd0 * TWO * POINT_ZERO_ONE);
        self.arr[64] = highd0 < (lowd0 + lowd0 * TWO_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[65] = highd0 < (lowd0 + lowd0 * THREE * POINT_ZERO_ONE);
        self.arr[66] = closed1 > closed2;
        self.arr[67] = closed1 < closed2;
        self.arr[68] = closed1 < opend1;
        self.arr[69] = closed1 > opend1;
        self.arr[70] = closed1 < (closed2 - closed2 * POINT_FIVE * POINT_ZERO_ONE);

        self.arr[71] = closed1 < (closed2 - closed2 * ONE * POINT_ZERO_ONE);
        self.arr[72] = closed1 < (closed2 - closed2 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[73] = closed1 < (closed2 - closed2 * TWO * POINT_ZERO_ONE);
        self.arr[74] = closed1 < (closed2 - closed2 * TWO_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[75] = closed1 < (closed2 - closed2 * THREE * POINT_ZERO_ONE);
        self.arr[76] = closed1 > (closed2 + closed2 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[77] = closed1 > (closed2 + closed2 * ONE * POINT_ZERO_ONE);
        self.arr[78] = closed1 > (closed2 + closed2 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[79] = closed1 > (closed2 + closed2 * TWO * POINT_ZERO_ONE);
        self.arr[80] = highd0 > highd1;

        self.arr[81] = highd0 > (highd1 + highd1 * POINT_TWO_FIVE * POINT_ZERO_ONE);
        self.arr[82] = highd0 > (highd1 + highd1 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[83] = highd0 > (highd1 + highd1 * POINT_SEVEN_FIVE * POINT_ZERO_ONE);
        self.arr[84] = highd0 > (highd1 + highd1 * ONE * POINT_ZERO_ONE);
        self.arr[85] = highd0 > (highd1 + highd1 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[86] = highd0 < highd1;
        self.arr[87] = highd0 < (highd1 - highd1 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[88] = highd0 < (highd1 - highd1 * ONE * POINT_ZERO_ONE);
        self.arr[89] = highd0 < (highd1 - highd1 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[90] = highd0 < (highd1 - highd1 * TWO * POINT_ZERO_ONE);

        self.arr[91] = highd0 < (highd1 - highd1 * TWO_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[92] = highd1 > highd5;
        self.arr[93] = highd1 < highd5;
        self.arr[94] = lowd0 < lowd1;
        self.arr[95] = lowd0 < (lowd1 - lowd1 * POINT_TWO_FIVE * POINT_ZERO_ONE);
        self.arr[96] = lowd0 < (lowd1 - lowd1 * POINT_FIVE * POINT_ZERO_ONE);
        self.arr[97] = lowd0 < (lowd1 - lowd1 * POINT_SEVEN_FIVE * POINT_ZERO_ONE);
        self.arr[98] = lowd0 < (lowd1 - lowd1 * ONE * POINT_ZERO_ONE);
        self.arr[99] = lowd0 > lowd1;
        self.arr[100] = lowd0 > (lowd1 + lowd1 * POINT_FIVE * POINT_ZERO_ONE);

        self.arr[101] = lowd0 > (lowd1 + lowd1 * ONE * POINT_ZERO_ONE);
        self.arr[102] = lowd0 > (lowd1 + lowd1 * ONE_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[103] = lowd0 > (lowd1 + lowd1 * TWO * POINT_ZERO_ONE);
        self.arr[104] = lowd0 > (lowd1 + lowd1 * TWO_POINT_FIVE * POINT_ZERO_ONE);
        self.arr[105] = lowd1 < lowd5;
        self.arr[106] = lowd1 > lowd5;
        self.arr[107] = (highd1 > highd2) && (highd1 > highd3) && (highd1 > highd4);
        self.arr[108] = (highd1 < highd2) && (highd1 < highd3) && (highd1 < highd4);
        self.arr[109] = (lowd1 < lowd2) && (lowd1 < lowd3) && (lowd1 < lowd4);
        self.arr[110] = (lowd1 > lowd2) && (lowd1 > lowd3) && (lowd1 > lowd4);

        self.arr[111] = (closed1 > closed2) && (closed2 > closed3) && (opend0 > closed1);
        self.arr[112] = (closed1 < closed2) && (closed2 < closed3) && (opend0 < closed1);
        self.arr[113] = (highd1 - closed1) < POINT_TWO * range1d;
        self.arr[114] = (closed1 - lowd1) < POINT_TWO * range1d;
        self.arr[115] = opend0 < lowd1 || opend0 > highd1;
        self.arr[116] = opend0 < lowd1;
        self.arr[117] = opend0 > highd1;
        self.arr[118] = opend0 < (closed1 - (closed1 * POINT_TWO_FIVE * POINT_ZERO_ONE));
        self.arr[119] = opend0 < (closed1 - (closed1 * POINT_FIVE * POINT_ZERO_ONE));
        self.arr[120] = opend0 < (closed1 - (closed1 * POINT_SEVEN_FIVE * POINT_ZERO_ONE));

        self.arr[121] = opend0 < (closed1 - (closed1 * ONE * POINT_ZERO_ONE));
        self.arr[122] = opend0 > (closed1 + (closed1 * POINT_TWO_FIVE * POINT_ZERO_ONE));
        self.arr[123] = opend0 > (closed1 + (closed1 * POINT_FIVE * POINT_ZERO_ONE));
        self.arr[124] = opend0 > (closed1 + (closed1 * POINT_SEVEN_FIVE * POINT_ZERO_ONE));
        self.arr[125] = opend0 > (closed1 + (closed1 * ONE * POINT_ZERO_ONE));
        self.arr[126] = (highd0 < highd1) && (lowd0 > lowd1);
        self.arr[127] = range1d < ((highd2 - lowd2) + (highd3 - lowd3) / THREE);
        self.arr[128] = (range1d < (highd2 - lowd2)) && ((highd2 - lowd2) < (highd3 - lowd3));
        self.arr[129] = (highd2 > highd1) && (lowd2 < lowd1);
        self.arr[130] = highd1 < highd2;

        self.arr[131] = lowd1 > lowd2;
        self.arr[132] = (highd1 < highd2) || (lowd1 > lowd2);
        self.arr[133] = (highd2 < highd1) && (lowd2 > lowd1);
        self.arr[134] = (highd0 > highd1) && (lowd0 < lowd1);
        self.arr[135] = (closed1 > opend1) && (closed2 > opend2);
        self.arr[136] = (closed1 < opend1) && (closed2 > opend2);
        self.arr[137] = (closed1 > opend1) && (closed2 < opend2);
        self.arr[138] = (closed1 < opend1) && (closed2 < opend2);

        Ok(())
    }
}
