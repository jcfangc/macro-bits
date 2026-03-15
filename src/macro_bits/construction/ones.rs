use crate::{WBLH, macro_bits::WideBits};

impl WideBits {
    #[inline]
    pub fn ones(len: usize) -> Self {
        let mut data = vec![u64::MAX; WBLH::required_word_len(len)];
        WBLH::sanitize_last_word(&mut data, len);
        Self::new_unchecked(len, data.into_boxed_slice())
    }
}

#[cfg(test)]
mod ones_tests {

    use super::*;
    use proptest::prelude::*;

    mod unit_tests {
        use super::*;

        #[test]
        fn zero_len() {
            let b = WideBits::ones(0);

            assert_eq!(b.len(), 0);
            assert!(b.data().is_empty());
        }

        #[test]
        fn small_lengths() {
            let cases = [1, 2, 3, 7, 8, 31, 32, 63, 64, 65, 127];

            for len in cases {
                let b = WideBits::ones(len);

                assert_eq!(b.len(), len);

                let expected = len.div_ceil(WBLH::WORD_BIT_WIDTH);
                assert_eq!(b.data().len(), expected);
            }
        }

        #[test]
        fn word_boundary() {
            let b = WideBits::ones(64);

            assert_eq!(b.data().len(), 1);
            assert_eq!(b.data()[0], u64::MAX);
        }

        #[test]
        fn tail_mask() {
            let b = WideBits::ones(10);

            assert_eq!(b.data().len(), 1);

            let expected = (1u64 << 10) - 1;
            assert_eq!(b.data()[0], expected);
        }

        #[test]
        fn multi_word() {
            let b = WideBits::ones(130);

            assert_eq!(b.data().len(), 3);

            assert_eq!(b.data()[0], u64::MAX);
            assert_eq!(b.data()[1], u64::MAX);

            let rem = 130 % 64;
            let mask = (1u64 << rem) - 1;

            assert_eq!(b.data()[2], mask);
        }
    }

    mod prop_tests {
        use super::*;

        proptest! {

            // word 数量公式
            #[test]
            fn word_len_formula(len in prop_oneof![
                Just(0usize),
                Just(1),
                Just(63),
                Just(64),
                Just(65),
                Just(127),
                0usize..256
            ]) {
                let b = WideBits::ones(len);

                let expected = len.div_ceil(WBLH::WORD_BIT_WIDTH);

                prop_assert_eq!(b.data().len(), expected);
            }

            // 尾 word mask 必须正确
            #[test]
            fn last_word_mask(len in prop_oneof![
                Just(0usize),
                Just(1),
                Just(63),
                Just(64),
                Just(65),
                Just(127),
                0usize..256
            ]) {
                let b = WideBits::ones(len);

                if len == 0 {
                    prop_assert!(b.data().is_empty());
                    return Ok(());
                }

                let rem = len % WBLH::WORD_BIT_WIDTH;

                let last = *b.data().last().unwrap();

                if rem == 0 {
                    prop_assert_eq!(last, u64::MAX);
                } else {
                    let mask = (1u64 << rem) - 1;
                    prop_assert_eq!(last, mask);
                }
            }
        }
    }
}
