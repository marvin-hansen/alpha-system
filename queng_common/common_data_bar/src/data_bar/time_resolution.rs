use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TimeResolution {
    #[default]
    NoValue = 0x0_u8,
    OneMin = 0x1_u8,
    FiveMin = 0x2_u8,
    FifteenMin = 0x3_u8,
    ThirtyMin = 0x4_u8,
    OneHour = 0x5_u8,
    OneDay = 0x6_u8,
    OneWeek = 0x7_u8,
    OneMonth = 0x8_u8,
    OneYear = 0x9_u8,
}

impl FromStr for TimeResolution {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoValue" => Ok(Self::NoValue),
            "OneMin" => Ok(Self::OneMin),
            "FiveMin" => Ok(Self::FiveMin),
            "FifteenMin" => Ok(Self::FifteenMin),
            "ThirtyMin" => Ok(Self::ThirtyMin),
            "OneHour" => Ok(Self::OneHour),
            "OneDay" => Ok(Self::OneDay),
            "OneWeek" => Ok(Self::OneWeek),
            "OneMonth" => Ok(Self::OneMonth),
            "OneYear" => Ok(Self::OneYear),
            _ => Err(()),
        }
    }
}

impl From<u8> for TimeResolution {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NoValue,
            0x1_u8 => Self::OneMin,
            0x2_u8 => Self::FiveMin,
            0x3_u8 => Self::FifteenMin,
            0x4_u8 => Self::ThirtyMin,
            0x5_u8 => Self::OneHour,
            0x6_u8 => Self::OneDay,
            0x7_u8 => Self::OneWeek,
            0x8_u8 => Self::OneMonth,
            0x9_u8 => Self::OneYear,
            _ => Self::NoValue,
        }
    }
}

impl fmt::Display for TimeResolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoValue => write!(f, "NoValue"),
            Self::OneMin => write!(f, "1 minute"),
            Self::FiveMin => write!(f, "5 minute"),
            Self::FifteenMin => write!(f, "15 minute"),
            Self::ThirtyMin => write!(f, "30 minute"),
            Self::OneHour => write!(f, "1 hour"),
            Self::OneDay => write!(f, "1 day"),
            Self::OneWeek => write!(f, "1 week"),
            Self::OneMonth => write!(f, "1 month"),
            Self::OneYear => write!(f, "1 year"),
        }
    }
}
