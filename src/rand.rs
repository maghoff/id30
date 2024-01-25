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
