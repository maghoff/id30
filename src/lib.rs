#![warn(missing_docs)]
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

//! Id30 is an encoding scheme for 30 bit identifiers that look like the
//! following: `bpv3uq`, `zvaec2`, `rfmbyz`, `jwygvk`, `000000`, `zzzzzz`. It is
//! designed for use as opaque identifiers in URLs, that can be read and written
//! comfortably.
//!
//! In machine readable form, Id30 is represented as a 32 bit integer, of which
//! the two most significant bits are always zero. This makes `u32` and `i32`
//! equally well suited.
//!
//! This crate defines the type [`Id30`] which implements this encoding scheme
//! through various traits. To avoid introducing excessive dependencies, several
//! of these trait implementations are opt-in via feature selection, see
//! [Features](#features) and [`Id30`] for details.
//!
//! # Id30 Encoding
//! The Id30 encoding is a case-insensitive base 32 encoding that handles some
//! confusable characters to compensate for some common misreadings and
//! mishearings. The canonical encoding alphabet is `0`, `1`, `2`, `3`, `4`,
//! `5`, `6`, `7`, `8`, `9`, `a`, `b`, `c`, `d`, `e`, `f`, `g`, `h`, `j`, `k`,
//! `m`, `n`, `p`, `q`, `r`, `t`, `u`, `v`, `w`, `x`, `y`, `z`, i.e. the digits
//! and then most, but not all, of the characters of the English alphabet. Id30
//! decoding handles case-insensitivity and confusable characters by mapping
//! multiple input characters to the same values.
//!
//! The confusable characters handled by Id30 are:
//!  - `0` and `o`/`O`, for visual similarity
//!  - `1`, `i`/`I` and `l`/`L`, for visual similarity
//!  - `s`/`S` and `f`/`F`, for verbal similarity
//!
//! ## Encoding
//! The algorithm for encoding a 30 bit integer `i` is:
//!  1. Group the integer in blocks of five bits, i.e. `let blocks = [(i >> 25)
//!     & 0x1f, (i >> 20) & 0x1f, (i >> 15) & 0x1f, (i >> 10) & 0x1f, (i >> 5)
//!     & 0x1f, i & 0x1f]`
//!  2. Starting with the most significant group of bits, map each group
//!     according to the canonical encoding alphabet, i.e. `let id30: String =
//!     blocks.iter().map(|x| ENCODING[x]).collect()`
//!
//! This yields a string of six characters. An Id30 string is always six
//! characters long, it is not valid to strip leading zeros as is normally done
//! for regular base 10 numbers.
//!
//! This encoding scheme ensures that the numerical ordering of the 30 bit
//! integers is the same as the lexicographical ordering of the encoded strings.
//!
//! ## Decoding
//! Decoding is more involved because of error handling and resolution of
//! alternative encodings.
//!
//! The algorithm for decoding an Id30 string to a 30 bit integer is:
//!  1. The given string is six characters long. Map each character according to
//!     the reverse mapping of the encoding. Additionally, map upper-case
//!     letters like their lower-case equivalents, and map the following
//!     confusable characters:
//!      * `o`, `O` → 0 (like `0`)
//!      * `i`, `I`, `l`, `L` → 1 (like `1`)
//!      * `s`, `S` → 15 (like `f`)
//!
//!     Characters that match the canonical encoding are mapped directly to the
//!     corresponding value, eg `'1' → 1`. For alternative encodings, either
//!     upper case or one of the confusables, add an additional bit (`1 << 5 ==
//!     0x20`) to signify that the input was non-canonical, eg `'I' → 1 + 0x20`.
//!     Map all other characters to a sentinel value (`1 << 6 == 0x40`) to
//!     signify decoding error, eg `'!' → 0x40`.
//!
//!     This can be encoded in a lookup table, so, given the input `id30:
//!     [u8; 6]`, we can implement this step as
//!     ```
//!     # const DECODING: [u32; 256] = [0; 256];
//!     # let id30: [u8; 6] = [0; 6];
//!     let decoded: Vec<_> = id30.iter().map(|&c| DECODING[c as usize]).collect();
//!     ```
//!  2. If any of the mapped values are the error sentinel, terminate decoding
//!     and yield an error result.
//!  3. If any of the mapped values have the alternative encoding bit set, let
//!     `is_canonical = false`. Otherwise, `is_canonical = true`.
//!  4. Compose the identifier value from the low five bits of all the mapped
//!     values:
//!     ```
//!     # let decoded = vec![0, 1, 2, 3, 4, 5];
//!     let masked: Vec<_> = decoded.iter().map(|x| x & 0x1f).collect();
//!     let value = (masked[0] << 25) + (masked[1] << 20) + (masked[2] << 15) +
//!         (masked[3] << 10) + (masked[4] << 5) + masked[5];
//!     ```
//!     Yield the composed value along with `is_canonical`.
//!
//! For use in URLs, a request to a non-canonical Id30 should be redirected to
//! the canonical encoding of the same Id30, to avoid exposing the same resource
//! at multiple different URLs. This is valuable for caching purposes and search
//! engine ranking. The redirection logic only needs to consider `is_canonical`
//! and does not need any costly operations such as querying a database, since
//! redirection can be done regardless of whether or not the target URL resolves
//! to anything sensible.
//!
//! ## Features
//! This crate uses features for selection of integrations with other crates.
//! Keeping the feature selection minimal serves to minimize the dependency
//! tree.
//!
//! To be able to support different major versions of third party crates, the
//! exposed features are named for the dependency version. For example, to
//! integrate with the `rand` crate in version 0.8.z, use the feature `rand08`.
//! This scheme allows future versions of `id30` to offer integrations with
//! multiple versions of `rand` simultaneously.
//!
//! As a convenience, there are unversioned aliases of each integration that
//! point to the latest supported major version of the given crate. For example,
//! using the feature `rand` enables integration with `rand` at the currently
//! supported latest version. `id30` does not consider it a breaking change to
//! update these aliases to point to newer versions, so you may want to consider
//! using the versioned feature names instead.
//!
//! There is one default feature, `rand`.
//!
//! The available integration features are:
//!  - `rand08` (alias `rand`), for integration with `rand` 0.8.z
//!  - `serde1` (alias `serde`), for integration with `serde` 1.y.z
//!  - `diesel2` (alias `diesel`), for integration with `diesel` 2.y.z
//!
//! See [`Id30`] for details about each integration.

mod codec_tables;
mod diesel_support;
mod display;
mod from;
mod from_str;
mod id30_parse;
mod rand;
mod serde_support;

use std::fmt::Debug;

pub use from::OutOfRangeError;
pub use id30_parse::{Id30Parse, ParseError};

#[cfg(feature = "diesel2")]
use diesel2 as diesel;
#[cfg(feature = "diesel2")]
use diesel2::sql_types::Integer;

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
///     # use id30::Id30;
///     # use rand08 as rand;
///     use rand::{distributions::Standard, prelude::*};
///     let mut rng = rand::thread_rng();
///
///     let id: Id30 = rng.gen();
///     let ids: Vec<Id30> = Standard.sample_iter(rng).take(10).collect();
///     ```
///  - With feature `serde`, via deserialization
///  - With feature `diesel`, as output from queries
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
///
/// # Integrations with other crates
///  - crate `rand` via feature `rand08` (alias `rand`):
///
///     `Id30` implements [`Distribution`], enabling generation of random `Id30`
///     values.
///
///  - crate `serde` via feature `serde1` (alias `serde`):
///
///     `Id30` implements [`Serialize`] and [`Deserialize`], enabling
///     serialization and deserialization of `Id30` values as text through
///     serde.
///
///  - crate `diesel` via feature `diesel2` (alias `diesel`):
///
///     `Id30` implements [`FromSql`] and [`Queryable`], enabling
///     deserialization of integers from the database as `Id30` values, and
///     [`ToSql`], enabling serialization of `Id30` values as integers in the
///     database. Additionally, [`AsExpression`] is implemented, enabling the
///     usage of `Id30` values in diesel query builder expressions.
///
/// [`Distribution`]: rand08::distributions::Distribution
///
/// [`Serialize`]: serde1::ser::Serialize
/// [`Deserialize`]: serde1::de::Deserialize
///
/// [`FromSql`]: diesel2::deserialize::FromSql
/// [`Queryable`]: diesel2::deserialize::Queryable
/// [`ToSql`]: diesel2::serialize::ToSql
/// [`AsExpression`]: diesel::expression::AsExpression

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "diesel2", derive(diesel::AsExpression, diesel::FromSqlRow))]
#[cfg_attr(feature = "diesel2", diesel(sql_type = Integer))]
pub struct Id30(u32);

impl Debug for Id30 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id30")
            .field("id30", &self.to_string())
            .field("u32", &self.0)
            .finish()
    }
}
