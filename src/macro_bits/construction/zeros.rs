use crate::{MBLH, macro_bits::MacroBits};

impl MacroBits {
    #[inline]
    pub fn zeros(len: usize) -> Self {
        let word_len = len.div_ceil(MBLH::WORD_BIT_WIDTH);
        let data = vec![0; word_len].into_boxed_slice();

        Self::new_unchecked(len, data)
    }
}

#[cfg(test)]
mod zeros_tests {

    use super::*;
    use proptest::prelude::*;

    mod unit_tests {
        use super::*;

        #[test]
        fn zero_len() {
            let b = MacroBits::zeros(0);

            assert_eq!(b.len(), 0);
            assert!(b.data().is_empty());
        }

        #[test]
        fn small_lengths() {
            let cases = [1, 2, 3, 7, 8, 31, 32, 63, 64, 65, 127];

            for len in cases {
                let b = MacroBits::zeros(len);

                assert_eq!(b.len(), len);

                let expected = len.div_ceil(MBLH::WORD_BIT_WIDTH);
                assert_eq!(b.data().len(), expected);

                assert!(b.data().iter().all(|&x| x == 0));
            }
        }

        #[test]
        fn word_boundary() {
            let b = MacroBits::zeros(64);

            assert_eq!(b.data().len(), 1);
            assert_eq!(b.data()[0], 0);
        }

        #[test]
        fn multi_word() {
            let b = MacroBits::zeros(130);

            assert_eq!(b.data().len(), 3);
            assert!(b.data().iter().all(|&x| x == 0));
        }
    }

    mod prop_tests {
        use super::*;

        proptest! {

            // 结构性质：所有 word 必须为 0
            #[test]
            fn all_words_zero(len in prop_oneof![
                Just(0usize),
                Just(1),
                Just(63),
                Just(64),
                Just(65),
                Just(127),
                0usize..512
            ]) {
                let b = MacroBits::zeros(len);

                prop_assert_eq!(b.len(), len);

                for &w in b.data() {
                    prop_assert_eq!(w, 0);
                }
            }

            // word 数量必须匹配数学公式
            #[test]
            fn word_len_formula(len in prop_oneof![
                Just(0usize),
                Just(1),
                Just(63),
                Just(64),
                Just(65),
                Just(127),
                0usize..512
            ]) {
                let b = MacroBits::zeros(len);

                let expected = len.div_ceil(MBLH::WORD_BIT_WIDTH);

                prop_assert_eq!(b.data().len(), expected);
            }
        }
    }
}
