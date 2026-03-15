use crate::{WBLH, WideBits};
use proptest::prelude::*;

#[derive(Copy, Clone, Debug)]
enum BinOp {
    And,
    Or,
    Xor,
    AndNot,
}

impl BinOp {
    #[inline]
    fn name(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::AndNot => "andnot",
        }
    }

    #[inline]
    fn apply_bit(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
            Self::AndNot => a & !b,
        }
    }

    #[inline]
    fn apply_public(self, lhs: &WideBits, rhs: &WideBits) -> WideBits {
        match self {
            Self::And => lhs.and(rhs),
            Self::Or => lhs.or(rhs),
            Self::Xor => lhs.xor(rhs),
            Self::AndNot => lhs.andnot(rhs),
        }
    }

    #[inline]
    fn apply_assign_public(self, lhs: &mut WideBits, rhs: &WideBits) {
        match self {
            Self::And => lhs.and_assign(rhs),
            Self::Or => lhs.or_assign(rhs),
            Self::Xor => lhs.xor_assign(rhs),
            Self::AndNot => lhs.andnot_assign(rhs),
        }
    }

    #[inline]
    fn apply_scalar(self, lhs: &WideBits, rhs: &WideBits) -> WideBits {
        match self {
            Self::And => lhs.and_scalar(rhs),
            Self::Or => lhs.or_scalar(rhs),
            Self::Xor => lhs.xor_scalar(rhs),
            Self::AndNot => lhs.andnot_scalar(rhs),
        }
    }

    #[inline]
    fn apply_assign_scalar(self, lhs: &mut WideBits, rhs: &WideBits) {
        match self {
            Self::And => lhs.and_assign_scalar(rhs),
            Self::Or => lhs.or_assign_scalar(rhs),
            Self::Xor => lhs.xor_assign_scalar(rhs),
            Self::AndNot => lhs.andnot_assign_scalar(rhs),
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    unsafe fn apply_avx2(self, lhs: &WideBits, rhs: &WideBits) -> WideBits {
        match self {
            Self::And => unsafe { lhs.and_avx2(rhs) },
            Self::Or => unsafe { lhs.or_avx2(rhs) },
            Self::Xor => unsafe { lhs.xor_avx2(rhs) },
            Self::AndNot => unsafe { lhs.andnot_avx2(rhs) },
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[inline]
    unsafe fn apply_assign_avx2(self, lhs: &mut WideBits, rhs: &WideBits) {
        match self {
            Self::And => unsafe { lhs.and_assign_avx2(rhs) },
            Self::Or => unsafe { lhs.or_assign_avx2(rhs) },
            Self::Xor => unsafe { lhs.xor_assign_avx2(rhs) },
            Self::AndNot => unsafe { lhs.andnot_assign_avx2(rhs) },
        }
    }
}

#[inline]
fn symmetric_ops() -> [BinOp; 3] {
    [BinOp::And, BinOp::Or, BinOp::Xor]
}

#[inline]
fn all_ops() -> [BinOp; 4] {
    [BinOp::And, BinOp::Or, BinOp::Xor, BinOp::AndNot]
}

#[inline]
fn boxed_words_for_len(len: usize) -> Box<[u64]> {
    vec![0u64; WBLH::required_word_len(len)].into_boxed_slice()
}

#[inline]
fn macro_bits_from_bools(bits: &[bool]) -> WideBits {
    let len = bits.len();
    let mut data = boxed_words_for_len(len);

    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            data[i / 64] |= 1u64 << (i % 64);
        }
    }

    WideBits::new_unchecked(len, data)
}

#[inline]
fn bools_from_macro_bits(x: &WideBits) -> Vec<bool> {
    (0..x.len)
        .map(|i| ((x.data[i / 64] >> (i % 64)) & 1) != 0)
        .collect()
}

#[inline]
fn zero_bits(len: usize) -> WideBits {
    WideBits::new_unchecked(len, boxed_words_for_len(len))
}

#[inline]
fn ref_apply(op: BinOp, lhs: &[bool], rhs: &[bool]) -> Vec<bool> {
    lhs.iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| op.apply_bit(a, b))
        .collect()
}

#[inline]
fn fixed_patterns(len: usize) -> Vec<Vec<bool>> {
    let zeros = vec![false; len];
    let ones = vec![true; len];
    let alt_10 = (0..len).map(|i| i % 2 == 0).collect::<Vec<_>>();
    let alt_01 = (0..len).map(|i| i % 2 == 1).collect::<Vec<_>>();

    let mut first = vec![false; len];
    if len > 0 {
        first[0] = true;
    }

    let mut last = vec![false; len];
    if len > 0 {
        last[len - 1] = true;
    }

    let mut every_3 = vec![false; len];
    for i in (0..len).step_by(3) {
        every_3[i] = true;
    }

    let mut every_5 = vec![false; len];
    for i in (0..len).step_by(5) {
        every_5[i] = true;
    }

    vec![zeros, ones, alt_10, alt_01, first, last, every_3, every_5]
}

#[inline]
fn special_lengths() -> [usize; 11] {
    [0, 1, 2, 63, 64, 65, 127, 128, 129, 255, 256]
}

#[inline]
fn boundary_lengths() -> [usize; 7] {
    [0, 63, 64, 65, 255, 256, 257]
}

#[inline]
fn assert_tail_sanitized(x: &WideBits) {
    let rem = x.len % 64;
    if rem == 0 || x.data.is_empty() {
        return;
    }

    let mask = (1u64 << rem) - 1;
    let last = *x.data.last().unwrap();
    assert_eq!(
        last & !mask,
        0,
        "tail bits must be zeroed: len={}, last={:#066b}, mask={:#066b}",
        x.len,
        last,
        mask
    );
}

#[test]
fn fixed_cases_match_reference_and_assign_and_len() {
    for op in all_ops() {
        for lhs_len in boundary_lengths() {
            for rhs_len in boundary_lengths() {
                let lhs_patterns = fixed_patterns(lhs_len);
                let rhs_patterns = fixed_patterns(rhs_len);

                for lhs_bits in &lhs_patterns {
                    for rhs_bits in &rhs_patterns {
                        let lhs = macro_bits_from_bools(lhs_bits);
                        let rhs = macro_bits_from_bools(rhs_bits);

                        let got = op.apply_public(&lhs, &rhs);
                        let want_bits = ref_apply(op, lhs_bits, rhs_bits);
                        let want = macro_bits_from_bools(&want_bits);

                        assert_eq!(got.len, lhs_len.min(rhs_len), "op={}", op.name());
                        assert_eq!(got, want, "op={}", op.name());
                        assert_tail_sanitized(&got);

                        let mut assigned = lhs.clone();
                        op.apply_assign_public(&mut assigned, &rhs);
                        assert_eq!(assigned.len, lhs_len.min(rhs_len), "op={}", op.name());
                        assert_eq!(assigned, want, "assign op={}", op.name());
                        assert_tail_sanitized(&assigned);
                    }
                }
            }
        }
    }
}

#[test]
fn self_identities_hold() {
    for len in special_lengths() {
        for bits in fixed_patterns(len) {
            let x = macro_bits_from_bools(&bits);

            let xx_and = x.and(&x);
            assert_eq!(xx_and, x, "x & x = x failed for len={len}");
            assert_tail_sanitized(&xx_and);

            let xx_or = x.or(&x);
            assert_eq!(xx_or, x, "x | x = x failed for len={len}");
            assert_tail_sanitized(&xx_or);

            let xx_xor = x.xor(&x);
            assert_eq!(xx_xor, zero_bits(len), "x ^ x = 0 failed for len={len}");
            assert_tail_sanitized(&xx_xor);

            let xx_andnot = x.andnot(&x);
            assert_eq!(xx_andnot, zero_bits(len), "x & !x = 0 failed for len={len}");
            assert_tail_sanitized(&xx_andnot);
        }
    }
}

#[test]
fn tail_word_is_sanitized_in_results() {
    let lens = [1usize, 63, 65, 127, 129, 191, 193, 255, 257];

    for op in all_ops() {
        for len in lens {
            let word_len = WBLH::required_word_len(len);
            let raw = vec![u64::MAX; word_len].into_boxed_slice();

            let lhs = WideBits::new_unchecked(len, raw.clone());
            let rhs = WideBits::new_unchecked(len, raw);

            let out = op.apply_public(&lhs, &rhs);
            assert_tail_sanitized(&out);

            let mut out_assign = lhs.clone();
            op.apply_assign_public(&mut out_assign, &rhs);
            assert_tail_sanitized(&out_assign);
        }
    }
}

proptest! {
    #[test]
    fn result_len_is_min_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..400),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..400),
    ) {
        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in all_ops() {
            let out = op.apply_public(&lhs, &rhs);
            prop_assert_eq!(out.len, lhs_bits.len().min(rhs_bits.len()), "op={}", op.name());
            assert_tail_sanitized(&out);
        }
    }

    #[test]
    fn assign_matches_non_assign_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..400),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..400),
    ) {
        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in all_ops() {
            let out = op.apply_public(&lhs, &rhs);

            let mut assigned = lhs.clone();
            op.apply_assign_public(&mut assigned, &rhs);

            prop_assert_eq!(assigned.clone(), out.clone(), "op={}", op.name());
            assert_tail_sanitized(&out);
            assert_tail_sanitized(&assigned);
        }
    }

    #[test]
    fn commutative_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..400),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..400),
    ) {
        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in symmetric_ops() {
            let ab = op.apply_public(&lhs, &rhs);
            let ba = op.apply_public(&rhs, &lhs);
            prop_assert_eq!(ab.clone(), ba.clone(), "op={}", op.name());
            assert_tail_sanitized(&ab);
            assert_tail_sanitized(&ba);
        }
    }

    #[test]
    fn reference_model_matches_public_api_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..400),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..400),
    ) {
        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in all_ops() {
            let got = op.apply_public(&lhs, &rhs);
            let want_bits = ref_apply(op, &lhs_bits, &rhs_bits);
            let want = macro_bits_from_bools(&want_bits);

            prop_assert_eq!(got.clone(), want, "op={}", op.name());
            assert_tail_sanitized(&got);
        }
    }

    #[test]
    fn scalar_backend_matches_reference_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..400),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..400),
    ) {
        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in all_ops() {
            let got = op.apply_scalar(&lhs, &rhs);
            let want_bits = ref_apply(op, &lhs_bits, &rhs_bits);
            let want = macro_bits_from_bools(&want_bits);

            prop_assert_eq!(got.clone(), want.clone(), "scalar op={}", op.name());
            assert_tail_sanitized(&got);

            let mut assigned = lhs.clone();
            op.apply_assign_scalar(&mut assigned, &rhs);
            prop_assert_eq!(assigned.clone(), want, "scalar assign op={}", op.name());
            assert_tail_sanitized(&assigned);
        }
    }
}

#[cfg(target_arch = "x86_64")]
proptest! {
    #[test]
    fn avx2_backend_matches_scalar_for_all_ops(
        lhs_bits in prop::collection::vec(any::<bool>(), 0..600),
        rhs_bits in prop::collection::vec(any::<bool>(), 0..600),
    ) {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return Ok(());
        }

        let lhs = macro_bits_from_bools(&lhs_bits);
        let rhs = macro_bits_from_bools(&rhs_bits);

        for op in all_ops() {
            let scalar = op.apply_scalar(&lhs, &rhs);
            let avx2 = unsafe { op.apply_avx2(&lhs, &rhs) };

            prop_assert_eq!(avx2.clone(), scalar.clone(), "avx2 op={}", op.name());
            assert_tail_sanitized(&scalar);
            assert_tail_sanitized(&avx2);

            let mut lhs_scalar_assign = lhs.clone();
            op.apply_assign_scalar(&mut lhs_scalar_assign, &rhs);

            let mut lhs_avx2_assign = lhs.clone();
            unsafe { op.apply_assign_avx2(&mut lhs_avx2_assign, &rhs) };

            prop_assert_eq!(lhs_avx2_assign.clone(), lhs_scalar_assign.clone(), "avx2 assign op={}", op.name());
            assert_tail_sanitized(&lhs_scalar_assign);
            assert_tail_sanitized(&lhs_avx2_assign);
        }
    }
}

#[test]
fn bool_roundtrip_smoke() {
    for len in special_lengths() {
        for bits in fixed_patterns(len) {
            let x = macro_bits_from_bools(&bits);
            let back = bools_from_macro_bits(&x);
            assert_eq!(back, bits, "len={len}");
        }
    }
}
