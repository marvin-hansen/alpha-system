use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PatternType {
    NullVal = 0xff_u8,
    #[default]
    Base = 0x1_u8,
    Extra = 0x2_u8,
    Long = 0x3_u8,
    Short = 0x4_u8,
}

impl From<u8> for PatternType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Base,
            0x2_u8 => Self::Extra,
            0x3_u8 => Self::Long,
            0x4_u8 => Self::Short,
            _ => Self::NullVal,
        }
    }
}

impl PatternType {
    pub fn get_pattern_type(&self) -> PatternType {
        match self {
            PatternType::NullVal => PatternType::NullVal,
            PatternType::Base => PatternType::Base,
            PatternType::Extra => PatternType::Extra,
            PatternType::Long => PatternType::Long,
            PatternType::Short => PatternType::Short,
        }
    }
}

impl Display for PatternType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::NullVal => write!(f, "NullVal"),
            PatternType::Base => write!(f, "Base"),
            PatternType::Extra => write!(f, "Extra"),
            PatternType::Long => write!(f, "Long"),
            PatternType::Short => write!(f, "Short"),
        }
    }
}
