use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum Architecture {
    Unknown = 0,
    Arm = 1,
    #[default]
    X86_64 = 2,
}

impl From<u8> for Architecture {
    fn from(value: u8) -> Self {
        match value {
            1 => Architecture::Arm,
            2 => Architecture::X86_64,
            _ => Architecture::Unknown,
        }
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::Unknown => write!(f, "Unknown"),
            Architecture::Arm => write!(f, "Arm"),
            Architecture::X86_64 => write!(f, "X86_64"),
        }
    }
}
