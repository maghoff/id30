use std::fmt::Display;

use crate::Id30;

pub(crate) mod generic {
    use crate::{codec_tables::ENCODE, Id30};

    use std::mem::MaybeUninit;

    #[allow(unused)]
    pub fn with_str<T>(id30: &Id30, f: impl FnOnce(&str) -> T) -> T {
        let mut buf = [MaybeUninit::uninit(); 6];

        buf.iter_mut()
            .zip([25, 20, 15, 10, 5, 0])
            .for_each(|(b, shift)| {
                b.write(ENCODE[((id30.0 >> shift) & 0b1_1111) as usize]);
            });

        // SAFETY:
        // - buf is utf8 because the ENCODE table is all ASCII
        // - transmute is safe because all elements have been initialized
        f(unsafe { std::str::from_utf8_unchecked(std::mem::transmute(buf.as_slice())) })
    }
}

#[cfg(feature = "unstable_stdarch_x86_avx512")]
pub(crate) mod avx512 {
    use crate::{codec_tables::ENCODE, Id30};

    use std::simd::prelude::*;

    #[allow(unused)]
    pub fn with_str<T>(id30: &Id30, f: impl FnOnce(&str) -> T) -> T {
        let x = u32x8::splat(id30.0);
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

        f(unsafe { std::str::from_utf8_unchecked(&buf[0..6]) })
    }
}

#[cfg(feature = "unstable_portable_simd")]
pub(crate) mod portable_simd {
    use crate::{codec_tables::ENCODE, Id30};

    use std::simd::prelude::*;

    #[allow(unused)]
    pub fn with_str<T>(id30: &Id30, f: impl FnOnce(&str) -> T) -> T {
        let x = u32x8::splat(id30.0);
        let shift = u32x8::from_array([25, 20, 15, 10, 5, 0, 30, 30]);

        let indices = x >> shift;

        // SAFETY Transmute between equally sized simd types is safe:
        let indices: u8x32 = unsafe { std::mem::transmute(indices) };

        // vpermb (avx512) only looks at the low 5 bits regardless, so optimization
        // should elide this mask. Unfortunately it doesn't manage to.
        let indices = indices & u8x32::splat(0b1_1111);

        let encode_table = u8x32::from_array(ENCODE);

        // swizzle_dyn gives abysmal performance unless `-Zbuild-std` is used
        // in the cargo invocation
        let encoded = encode_table.swizzle_dyn(indices);

        // SAFETY Transmute between equally sized simd types is safe:
        let encoded: u32x8 = unsafe { std::mem::transmute(encoded) };

        let buf = encoded.cast::<u8>().to_array();

        f(unsafe { std::str::from_utf8_unchecked(&buf[0..6]) })
    }
}

#[allow(unreachable_code)]
fn with_str<T>(id30: &Id30, f: impl FnOnce(&str) -> T) -> T {
    #[cfg(feature = "unstable_portable_simd")]
    return portable_simd::with_str(id30, f);

    #[cfg(feature = "unstable_stdarch_x86_avx512")]
    return avx512::with_str(id30, f);

    generic::with_str(id30, f)
}

impl Display for Id30 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        with_str(self, move |id30_str| f.write_str(id30_str))
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
