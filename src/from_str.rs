use std::str::FromStr;

use crate::Id30;

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

mod generic {
    use crate::codec_tables::{ALT_MASK, DECODE, ERR_MASK};

    use super::*;

    #[allow(unused)]
    #[inline(always)]
    pub fn from_str(s: &str) -> Result<Id30Parse, ParseError> {
        let s: &[u8; 6] = s
            .as_bytes()
            .try_into()
            .map_err(|_| ParseError::InvalidLength)?;

        let mut decoded = [0u8; 8];
        decoded
            .iter_mut()
            .zip(s)
            .for_each(|(b, i)| *b = DECODE[*i as usize]);

        let dec_u64 = u64::from_le_bytes(decoded);

        if dec_u64 & ERR_MASK != 0 {
            return Err(ParseError::InvalidCharacters);
        }

        let canonical = if dec_u64 & ALT_MASK != 0 {
            Canonical::Alternate
        } else {
            Canonical::Canonical
        };

        let value = decoded
            .iter()
            .zip([25, 20, 15, 10, 5, 0])
            .take(6)
            .map(|(b, shift)| ((b & 0b1_1111) as u32) << shift)
            .fold(0, |a, b| a | b);

        Ok(Id30Parse {
            id30: Id30(value),
            canonical,
        })
    }
}

#[cfg(feature = "unstable_portable_simd")]
mod portable_simd {
    use std::simd::prelude::*;

    #[allow(unused)]
    use crate::codec_tables::{
        ALT_FLAG, ALT_MASK, DECODE, DECODE_HIGH, DECODE_LOW, ERR_FLAG, ERR_MASK,
    };

    use super::*;

    #[inline(always)]
    pub fn from_str(s: &str) -> Result<Id30Parse, ParseError> {
        let s: &[u8; 6] = s
            .as_bytes()
            .try_into()
            .map_err(|_| ParseError::InvalidLength)?;

        let mut s2 = [b'0'; 64];
        s2[0..6].copy_from_slice(s);
        let zmm = Simd::from(s2);
        let xmm = simd_swizzle!(zmm, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

        if xmm & Simd::splat(0b1000_0000) != Simd::splat(0) {
            return Err(ParseError::InvalidCharacters);
        }

        let high64_mask = (zmm & u8x64::splat(0b0100_0000)).simd_ne(Simd::splat(0));
        let decoded: u8x64 = unsafe {
            use std::arch::x86_64::*;

            let decode_low = u8x64::from(DECODE_LOW);
            let low = _mm512_permutexvar_epi8(zmm.into(), decode_low.into());

            let decode_high = u8x64::from(DECODE_HIGH);
            let high = _mm512_permutexvar_epi8(zmm.into(), decode_high.into());

            high64_mask.select(high.into(), low.into())
        };

        let dec_u64 = u64::from_le_bytes(decoded.as_array()[0..8].try_into().unwrap());

        if dec_u64 & ERR_MASK != 0 {
            return Err(ParseError::InvalidCharacters);
        }

        let canonical = if dec_u64 & ALT_MASK != 0 {
            Canonical::Alternate
        } else {
            Canonical::Canonical
        };

        let value = u8x8::from_array(decoded.to_array()[0..8].try_into().unwrap());
        let value: u32x8 = value.cast();

        let shift = u32x8::from_array([25, 20, 15, 10, 5, 0, 32, 32]);

        let value = (value & u32x8::splat(0b1_1111)) << shift;

        let value = value | simd_swizzle!(value, [1, 0, 3, 2, 5, 4, 7, 6]);
        let value = value[0] | value[2] | value[4];

        Ok(Id30Parse {
            id30: Id30(value),
            canonical,
        })
    }
}

#[cfg(not(feature = "unstable_portable_simd"))]
impl FromStr for Id30Parse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        generic::from_str(s)
    }
}

#[cfg(feature = "unstable_portable_simd")]
impl FromStr for Id30Parse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        portable_simd::from_str(s)
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
