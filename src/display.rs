#[cfg(not(feature = "unstable_portable_simd"))]
pub(crate) mod generic {
    use crate::{codec_tables::ENCODE, Id30};

    use std::{fmt::Display, mem::MaybeUninit};

    impl Display for Id30 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut buf = [MaybeUninit::uninit(); 6];

            buf.iter_mut()
                .zip([25, 20, 15, 10, 5, 0])
                .for_each(|(b, shift)| {
                    b.write(ENCODE[((self.0 >> shift) & 0b1_1111) as usize]);
                });

            // SAFETY:
            // - buf is utf8 because the ENCODE table is all ASCII
            // - transmute is safe because all elements have been initialized
            f.write_str(unsafe {
                std::str::from_utf8_unchecked(std::mem::transmute(buf.as_slice()))
            })
        }
    }
}

#[cfg(feature = "unstable_portable_simd")]
pub(crate) mod portable_simd {
    use crate::{codec_tables::ENCODE, Id30};

    use std::{fmt::Display, simd::prelude::*};

    impl Display for Id30 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let x = u32x8::splat(self.0);
            let shift = u32x8::from_array([25, 20, 15, 10, 5, 0, 30, 30]);

            let indices = x >> shift;

            // vpermb (via _mm256_permutexvar_epi8) only looks at the low
            // 5 bits regardless, so we don't need to mask off the others:
            // let indices = indices & u32x8::splat(0b1_1111);

            let encode_table = u8x32::from_array(ENCODE);

            // swizzle_dyn miscompiles on x86-64: https://github.com/rust-lang/rust/issues/119904
            // let buf = encode_table.swizzle_dyn(indices).to_array();
            //
            // Use intrinsic directly instead:
            let encoded = u32x8::from(unsafe {
                std::arch::x86_64::_mm256_permutexvar_epi8(indices.into(), encode_table.into())
            });

            // portable_simd for vpmovdb, but it retains the full width of the simd register
            // let buf = encoded.cast::<u8>().to_array();

            // vpmovdb, like above, but with xmm target instead of ymm
            // let buf =
            //     u8x16::from(unsafe { std::arch::x86_64::_mm256_cvtepi32_epi8(encoded.into()) })
            //         .to_array();

            // vpmovdb, with m64 target and mandatory mask (which gets eliminated in optimization)
            let mut buf = [0u8; 8];
            unsafe {
                std::arch::x86_64::_mm256_mask_cvtepi32_storeu_epi8(
                    std::mem::transmute(buf.as_mut_ptr()),
                    0b1111_1111,
                    encoded.into(),
                );
            };

            f.write_str(unsafe { std::str::from_utf8_unchecked(&buf[0..6]) })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Id30;

    #[test]
    fn display() {
        assert_eq!(Id30(0).to_string(), "000000");
        assert_eq!(Id30(1).to_string(), "000001");
        assert_eq!(Id30((1 << 30) - 1).to_string(), "zzzzzz");

        assert_eq!(Id30(347485647).to_string(), "abcdef");
    }
}
