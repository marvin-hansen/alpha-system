use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Architecture {
    Unknown = 0,
    Arm = 1,
    #[default]
    X86_64 = 2,
}

impl From<u8> for Architecture {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Arm,
            2 => Self::X86_64,
            _ => Self::Unknown,
        }
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Arm => write!(f, "Arm"),
            Self::X86_64 => write!(f, "X86_64"),
        }
    }
}
