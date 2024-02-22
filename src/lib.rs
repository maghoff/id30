#![cfg_attr(
    any(
        feature = "unstable_portable_simd",
        feature = "unstable_stdarch_x86_avx512"
    ),
    feature(portable_simd)
)]
#![cfg_attr(
    any(feature = "unstable_stdarch_x86_avx512"),
    feature(stdarch_x86_avx512)
)]

mod codec_tables;
mod display;
mod from;
mod from_str;
mod rand;

pub use from_str::{Canonical, Id30Parse, ParseError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Id30(u32);
