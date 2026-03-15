#[cfg(test)]
mod binary_op_tests;

mod avx2;
mod scalar;

use crate::MacroBits;

impl MacroBits {
    #[inline]
    pub fn and(&self, rhs: &Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                return unsafe { self.and_avx2(rhs) };
            }
        }

        self.and_scalar(rhs)
    }

    #[inline]
    pub fn and_assign(&mut self, rhs: &Self) {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                unsafe { self.and_assign_avx2(rhs) };
                return;
            }
        }

        self.and_assign_scalar(rhs);
    }

    #[inline]
    pub fn or(&self, rhs: &Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                return unsafe { self.or_avx2(rhs) };
            }
        }

        self.or_scalar(rhs)
    }

    #[inline]
    pub fn or_assign(&mut self, rhs: &Self) {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                unsafe { self.or_assign_avx2(rhs) };
                return;
            }
        }

        self.or_assign_scalar(rhs);
    }

    #[inline]
    pub fn xor(&self, rhs: &Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                return unsafe { self.xor_avx2(rhs) };
            }
        }

        self.xor_scalar(rhs)
    }

    #[inline]
    pub fn xor_assign(&mut self, rhs: &Self) {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: runtime-gated by AVX2 feature detection.
                unsafe { self.xor_assign_avx2(rhs) };
                return;
            }
        }

        self.xor_assign_scalar(rhs);
    }

    #[inline]
    pub fn andnot(&self, rhs: &Self) -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                return unsafe { self.andnot_avx2(rhs) };
            }
        }

        self.andnot_scalar(rhs)
    }

    #[inline]
    pub fn andnot_assign(&mut self, rhs: &Self) {
        #[cfg(target_arch = "x86_64")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                unsafe { self.andnot_assign_avx2(rhs) };
                return;
            }
        }

        self.andnot_assign_scalar(rhs);
    }
}
