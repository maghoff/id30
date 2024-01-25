use std::str::FromStr;

use crate::{
    codec_tables::{ALT_FLAG, DECODE, ERR_FLAG},
    Id30,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Canonical {
    Canonical,
    Alternate,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Id30Parse {
    pub id30: Id30,
    pub canonical: Canonical,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidLength,
    InvalidCharacters,
}

impl FromStr for Id30Parse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: &[u8; 6] = s
            .as_bytes()
            .try_into()
            .map_err(|_| ParseError::InvalidLength)?;

        let mut decoded = [0u8; 6];
        decoded
            .iter_mut()
            .zip(s)
            .for_each(|(b, i)| *b = DECODE[*i as usize]);

        let flags = decoded.iter().fold(0, |a, b| a | b);

        if flags & ERR_FLAG != 0 {
            return Err(ParseError::InvalidCharacters);
        }

        let canonical = if flags & ALT_FLAG != 0 {
            Canonical::Alternate
        } else {
            Canonical::Canonical
        };

        let value = decoded
            .iter()
            .zip([25, 20, 15, 10, 5, 0])
            .map(|(b, shift)| ((b & 0b1_1111) as u32) << shift)
            .fold(0, |a, b| a | b);

        Ok(Id30Parse {
            id30: Id30(value),
            canonical,
        })
    }
}

impl FromStr for Id30 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Id30Parse::from_str(s).map(|x| x.id30)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_id30() {
        assert_eq!("000000".parse(), Ok(Id30(0)));
        assert_eq!("000001".parse(), Ok(Id30(1)));
        assert_eq!("zzzzzz".parse(), Ok(Id30((1 << 30) - 1)));

        assert_eq!("abcdef".parse(), Ok(Id30(347485647)));
        assert_eq!("ABCDEF".parse(), Ok(Id30(347485647)));

        assert_eq!("0oO0oO".parse(), Ok(Id30(0)));
        assert_eq!("1lLiI1".parse(), Ok(Id30(34636833)));

        assert_eq!("00000u".parse::<Id30>(), Err(ParseError::InvalidCharacters));
        assert_eq!("00000".parse::<Id30>(), Err(ParseError::InvalidLength));
        assert_eq!("0000000".parse::<Id30>(), Err(ParseError::InvalidLength));
    }

    #[test]
    fn parse_id30parse() {
        assert!(matches!(
            "000000".parse(),
            Ok(Id30Parse {
                id30: Id30(0),
                canonical: Canonical::Canonical
            })
        ));
        assert!(matches!(
            "00oo00".parse(),
            Ok(Id30Parse {
                id30: Id30(0),
                canonical: Canonical::Alternate
            })
        ));
    }
}
