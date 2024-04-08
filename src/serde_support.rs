// Copyright 2024 Magnus Hovland Hoff.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/license/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "serde1")]

use serde1::{de, Deserialize, Serialize};

use crate::{display::generic::with_str, Id30, Id30Parse};

struct Id30Visitor;

impl<'de> de::Visitor<'de> for Id30Visitor {
    type Value = Id30;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid Id30 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse()
            .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for Id30 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde1::Deserializer<'de>,
    {
        deserializer.deserialize_str(Id30Visitor)
    }
}

struct Id30ParseVisitor;

impl<'de> de::Visitor<'de> for Id30ParseVisitor {
    type Value = Id30Parse;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid Id30 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse()
            .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
    }
}

impl<'de> Deserialize<'de> for Id30Parse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde1::Deserializer<'de>,
    {
        deserializer.deserialize_str(Id30ParseVisitor)
    }
}

impl Serialize for Id30 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde1::Serializer,
    {
        with_str(self, move |id30_str| serializer.serialize_str(id30_str))
    }
}

#[cfg(test)]
mod test {
    use serde_test1::{assert_de_tokens, assert_tokens, Token};

    use crate::{Id30, Id30Parse};

    #[test]
    fn deserialize_id30() {
        assert_tokens(
            &Id30::try_from(0x3fff_ffff).unwrap(),
            &[Token::Str("zzzzzz")],
        )
    }

    #[test]
    fn deserialize_id30parse() {
        assert_de_tokens(
            &Id30Parse {
                id30: Id30::try_from(0x3fff_ffff).unwrap(),
                is_canonical: true,
            },
            &[Token::Str("zzzzzz")],
        );

        assert_de_tokens(
            &Id30Parse {
                id30: Id30::try_from(0x3fff_ffff).unwrap(),
                is_canonical: false,
            },
            &[Token::Str("zzzZZZ")],
        );
    }
}
