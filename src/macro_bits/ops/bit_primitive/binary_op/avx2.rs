#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{WBLH, WideBits};

#[cfg(target_arch = "x86_64")]
type Avx2BinOp = unsafe fn(__m256i, __m256i) -> __m256i;

#[cfg(target_arch = "x86_64")]
impl WideBits {
    #[target_feature(enable = "avx2")]
    unsafe fn binary_op_avx2_words_to<F>(
        lhs: *const u64,
        rhs: *const u64,
        dst: *mut u64,
        word_len: usize,
        simd_op: Avx2BinOp,
        scalar_op: F,
    ) where
        F: Fn(u64, u64) -> u64,
    {
        let chunks = word_len / 4;
        let simd_words = chunks * 4;

        for i in 0..chunks {
            let off = i * 4;
            let a = unsafe { _mm256_loadu_si256(lhs.add(off) as *const __m256i) };
            let b = unsafe { _mm256_loadu_si256(rhs.add(off) as *const __m256i) };
            let r = unsafe { simd_op(a, b) };
            unsafe { _mm256_storeu_si256(dst.add(off) as *mut __m256i, r) };
        }

        for i in simd_words..word_len {
            unsafe {
                *dst.add(i) = scalar_op(*lhs.add(i), *rhs.add(i));
            }
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn binary_op_avx2_kernel<F>(
        lhs: &Self,
        rhs: &Self,
        simd_op: Avx2BinOp,
        scalar_op: F,
    ) -> (usize, Box<[u64]>)
    where
        F: Fn(u64, u64) -> u64,
    {
        let len = lhs.len.min(rhs.len);
        let word_len = WBLH::required_word_len(len);
        let mut data = vec![0u64; word_len];

        unsafe {
            Self::binary_op_avx2_words_to(
                lhs.data.as_ptr(),
                rhs.data.as_ptr(),
                data.as_mut_ptr(),
                word_len,
                simd_op,
                scalar_op,
            );
        }

        let mut data = data.into_boxed_slice();
        WBLH::sanitize_last_word(&mut data, len);
        (len, data)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn binary_op_avx2<F>(&self, rhs: &Self, simd_op: Avx2BinOp, scalar_op: F) -> Self
    where
        F: Fn(u64, u64) -> u64,
    {
        let (len, data) = unsafe { Self::binary_op_avx2_kernel(self, rhs, simd_op, scalar_op) };
        Self::new_unchecked(len, data)
    }

    #[target_feature(enable = "avx2")]
    unsafe fn binary_op_assign_avx2<F>(&mut self, rhs: &Self, simd_op: Avx2BinOp, scalar_op: F)
    where
        F: Fn(u64, u64) -> u64,
    {
        let (len, data) = unsafe { Self::binary_op_avx2_kernel(self, rhs, simd_op, scalar_op) };
        self.len = len;
        self.data = data;
    }
}

#[cfg(target_arch = "x86_64")]
impl WideBits {
    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn and_avx2(&self, rhs: &Self) -> Self {
        unsafe { self.binary_op_avx2(rhs, _mm256_and_si256, |x, y| x & y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn and_assign_avx2(&mut self, rhs: &Self) {
        unsafe { self.binary_op_assign_avx2(rhs, _mm256_and_si256, |x, y| x & y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn or_avx2(&self, rhs: &Self) -> Self {
        unsafe { self.binary_op_avx2(rhs, _mm256_or_si256, |x, y| x | y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn or_assign_avx2(&mut self, rhs: &Self) {
        unsafe { self.binary_op_assign_avx2(rhs, _mm256_or_si256, |x, y| x | y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn xor_avx2(&self, rhs: &Self) -> Self {
        unsafe { self.binary_op_avx2(rhs, _mm256_xor_si256, |x, y| x ^ y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn xor_assign_avx2(&mut self, rhs: &Self) {
        unsafe { self.binary_op_assign_avx2(rhs, _mm256_xor_si256, |x, y| x ^ y) }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn andnot_avx2(&self, rhs: &Self) -> Self {
        unsafe {
            self.binary_op_avx2(
                rhs,
                // _mm256_andnot_si256(a, b) == (!a) & b
                |a, b| _mm256_andnot_si256(b, a), // self & !rhs == (!rhs) & self
                |x, y| x & !y,
            )
        }
    }

    #[target_feature(enable = "avx2")]
    pub(super) unsafe fn andnot_assign_avx2(&mut self, rhs: &Self) {
        unsafe {
            self.binary_op_assign_avx2(
                rhs,
                // _mm256_andnot_si256(a, b) == (!a) & b
                |a, b| _mm256_andnot_si256(b, a), // self & !rhs == (!rhs) & self
                |x, y| x & !y,
            )
        }
    }
}
