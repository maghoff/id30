#![cfg_attr(feature = "unstable_portable_simd", feature(portable_simd, stdsimd))]

mod codec_tables;
mod display;
mod from;
mod from_str;
mod rand;

pub use from_str::{Canonical, Id30Parse, ParseError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Id30(u32);
