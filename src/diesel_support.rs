// Copyright 2024 Magnus Hovland Hoff.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/license/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "diesel2")]

use diesel2::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::Integer,
};

use crate::Id30;

impl<DB> ToSql<Integer, DB> for Id30
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> serialize::Result {
        // SAFETY: u32 and i32 have the same size and alignment
        let inner: &i32 = unsafe { std::mem::transmute(&self.0) };

        ToSql::<Integer, DB>::to_sql(inner, out)
    }
}

impl<DB> FromSql<Integer, DB> for Id30
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let num = i32::from_sql(bytes)?;
        Ok(Id30::try_from(num)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use diesel2 as diesel;

    use diesel2::prelude::*;
    use diesel2::sql_query;
    use std::error::Error;

    #[test]
    fn to_sql() -> Result<(), Box<dyn Error>> {
        let mut conn = SqliteConnection::establish(":memory:")?;

        #[derive(QueryableByName, PartialEq, Eq, Debug)]
        struct Row {
            #[diesel(sql_type = Integer)]
            int: i32,
        }

        let res = sql_query("SELECT ? as int")
            .bind::<Integer, _>(Id30::try_from(0x1234_5678).unwrap())
            .load::<Row>(&mut conn)?;

        assert_eq!(&[Row { int: 0x1234_5678 }], res.as_slice());

        Ok(())
    }

    #[test]
    fn from_sql() -> Result<(), Box<dyn Error>> {
        let mut conn = SqliteConnection::establish(":memory:")?;

        #[derive(QueryableByName, PartialEq, Eq, Debug)]
        struct Row {
            #[diesel(sql_type = Integer)]
            id30: Id30,
        }

        let res = sql_query("SELECT 0x12345678 as id30").load::<Row>(&mut conn)?;

        assert_eq!(
            &[Row {
                id30: Id30::try_from(0x1234_5678).unwrap()
            }],
            res.as_slice()
        );

        Ok(())
    }

    #[test]
    fn db_invalid_value_gives_error() -> Result<(), Box<dyn Error>> {
        let mut conn = SqliteConnection::establish(":memory:")?;

        #[derive(QueryableByName, PartialEq, Eq, Debug)]
        struct Row {
            #[diesel(sql_type = Integer)]
            id30: Id30,
        }

        let res = sql_query("SELECT 0x7fffffff as id30").load::<Row>(&mut conn);
        assert!(res.is_err());

        Ok(())
    }
}
