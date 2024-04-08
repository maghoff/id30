// Copyright 2024 Magnus Hovland Hoff.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/license/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "rand08")]
mod rand08_impl {
    use rand08::{
        distributions::{Distribution, Standard},
        Rng,
    };

    use crate::Id30;

    impl Distribution<Id30> for Standard {
        #[inline]
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Id30 {
            // Typically, if there is a bias, the highest quality random bits
            // are the high bits, so we shift the value into range:
            Id30(rng.next_u32() >> 2)
        }
    }
}
