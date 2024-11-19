use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    #[must_use]
    pub const fn get_pattern_type(&self) -> Self {
        match self {
            Self::NullVal => Self::NullVal,
            Self::Base => Self::Base,
            Self::Extra => Self::Extra,
            Self::Long => Self::Long,
            Self::Short => Self::Short,
        }
    }
}

impl Display for PatternType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullVal => write!(f, "NullVal"),
            Self::Base => write!(f, "Base"),
            Self::Extra => write!(f, "Extra"),
            Self::Long => write!(f, "Long"),
            Self::Short => write!(f, "Short"),
        }
    }
}
