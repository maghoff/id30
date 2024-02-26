use std::fmt;

use super::Id30;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        "value out of range for Id30".fmt(fmt)
    }
}

impl std::error::Error for OutOfRangeError {}

impl TryFrom<u32> for Id30 {
    type Error = OutOfRangeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 1 << 30 {
            Ok(Id30(value))
        } else {
            Err(OutOfRangeError)
        }
    }
}

impl TryFrom<i32> for Id30 {
    type Error = OutOfRangeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Id30::try_from(value as u32)
        } else {
            Err(OutOfRangeError)
        }
    }
}

impl From<Id30> for u32 {
    fn from(value: Id30) -> Self {
        value.0
    }
}

impl From<Id30> for i32 {
    fn from(value: Id30) -> Self {
        value.0 as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from_good_range() {
        assert!(Id30::try_from(0u32).is_ok());
        assert!(Id30::try_from(((1 << 30) - 1) as u32).is_ok());
        assert!(Id30::try_from(0x3fff_ffffu32).is_ok());
        assert!(Id30::try_from(0x1234_5678u32).is_ok());

        assert!(Id30::try_from(0i32).is_ok());
        assert!(Id30::try_from(((1 << 30) - 1) as i32).is_ok());
        assert!(Id30::try_from(0x3fff_ffffi32).is_ok());
        assert!(Id30::try_from(0x1234_5678i32).is_ok());
    }

    #[test]
    fn try_from_overflow_range() {
        assert!(Id30::try_from(0xffff_ffffu32).is_err());
        assert!(Id30::try_from(0x4000_0000).is_err());

        assert!(Id30::try_from(-0x8000_0000).is_err());
        assert!(Id30::try_from(-1).is_err());
    }
}
