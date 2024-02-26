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

//! Id30 is an encoding scheme for 30 bit identifiers that look like these:
//! `dw94m9`, `482s8w`, `7kqz08`, `mbrdfb`, `000000`, `zzzzzz`
//!
//! In machine readable form, Id30 is represented as a 32 bit integer, of which
//! the two most significant bits are always zero. This makes `u32` and `i32`
//! equally well suited.
//!
//! This crate defines the type [`Id30`] which implements this encoding scheme
//! through various traits. To avoid introducing excessive dependencies, most
//! of these trait implementations are opt-in via feature selection.

mod codec_tables;
mod display;
mod from;
mod from_str;
mod rand;

use std::fmt::Debug;

pub use from::OutOfRangeError;
pub use from_str::{Canonical, Id30Parse, ParseError};

/// An implementation of the Id30 encoding scheme as documented at the [crate]
/// root.
///
/// This type holds a 30 bit integer as a 32 bit integer, of which the two most
/// significant bits are always zero. The type has the same representation as
/// `u32`. Debug-formatting includes both the Id30 encoding and the integer
/// representation.
///
/// Ordering, as implemented for the `PartialOrd` and `Ord` traits, orders by
/// numerical value of the underlying integer. This is the same order as the
/// lexicographical ordering of the string representation.
///
/// There are many ways to create instances of `Id30`:
///  - Via the `TryFrom` trait from either a `u32` or an `i32`:
///     ```rust
///     # use id30::Id30;
///     let id: Id30 = 1234.try_into().expect("1234 is in range");
///     assert_eq!(Id30::try_from(1 << 31), Err(id30::OutOfRangeError));
///     ```
///  - Parsing, via the `FromStr` trait:
///     ```rust
///     # use id30::Id30;
///     let id: id30::Id30 = "hrga2q".parse().unwrap();
///     ```
///     **Tip:** Parse to the [`Id30Parse`] type if you need to know whether the
///     text representation is canonical or alternate.
///  - With feature `rand`, via the [`Distribution`][rand08::distributions::Distribution]
///     trait:
///     ```rust
///     # #[cfg(all(feature = "rand08", feature = "rand08_std", feature = "rand08_std_rng"))]
///     # {
///     # use id30::Id30;
///     # use rand08 as rand;
///     use rand::{distributions::Standard, prelude::*};
///     let mut rng = rand::thread_rng();
///
///     let id: Id30 = rng.gen();
///     let ids: Vec<Id30> = Standard.sample_iter(rng).take(10).collect();
///     # }
///     ```
///
/// An `Id30` can be converted directly to `u32` and `i32` via the `From` trait:
/// ```rust
/// # use id30::Id30;
/// let id: Id30 = 1234.try_into().unwrap();
/// assert_eq!(u32::from(id), 1234);
/// ```
///
/// `Id30` implements the `Display` trait:
/// ```rust
/// # use id30::Id30;
/// let id: id30::Id30 = "j9yceq".parse().unwrap();
/// assert_eq!(&id.to_string(), "j9yceq");
/// assert_eq!(&format!("/path/to/{id}"), "/path/to/j9yceq");
/// ```
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Id30(u32);

impl Debug for Id30 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id30")
            .field("id30", &self.to_string())
            .field("u32", &self.0)
            .finish()
    }
}
