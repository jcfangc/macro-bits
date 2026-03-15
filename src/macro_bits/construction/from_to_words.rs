use crate::{MBLH, macro_bits::MacroBits};

impl MacroBits {
    #[inline]
    pub fn from_words(words: &[u64], len: usize) -> Self {
        let mut data = words.to_vec().into_boxed_slice();
        MBLH::sanitize_last(&mut data, len);
        Self::new_unchecked(len, data)
    }

    #[inline]
    pub fn from_words_boxed(mut data: Box<[u64]>, len: usize) -> Self {
        MBLH::sanitize_last(&mut data, len);
        Self::new_unchecked(len, data)
    }
}

#[cfg(test)]
mod from_words_tests {

    use super::*;
    use proptest::prelude::*;

    mod unit_tests {
        use super::*;

        #[test]
        fn zero_len() {
            let words = [123u64, 456];
            let b = MacroBits::from_words(&words, 0);

            assert_eq!(b.len(), 0);
            assert_eq!(b.data().len(), 2);

            // tail mask 应该变成 MAX
            assert_eq!(b.data()[0], words[0]);
            assert_eq!(b.data()[1], words[1]);
        }

        #[test]
        fn exact_word_boundary() {
            let words = [u64::MAX, u64::MAX];
            let b = MacroBits::from_words(&words, 128);

            assert_eq!(b.len(), 128);
            assert_eq!(b.data(), &words);
        }

        #[test]
        fn tail_mask_applied() {
            let words = [u64::MAX];
            let b = MacroBits::from_words(&words, 10);

            let expected = (1u64 << 10) - 1;

            assert_eq!(b.len(), 10);
            assert_eq!(b.data()[0], expected);
        }

        #[test]
        fn multi_word_tail_mask() {
            let words = [u64::MAX, u64::MAX];
            let b = MacroBits::from_words(&words, 70);

            assert_eq!(b.data()[0], u64::MAX);

            let rem = 70 % 64;
            let mask = (1u64 << rem) - 1;

            assert_eq!(b.data()[1], mask);
        }
    }

    mod prop_tests {
        use super::*;

        proptest! {

            // len 必须被正确保存
            #[test]
            fn len_preserved(
                len in 0usize..512,
                words in prop::collection::vec(any::<u64>(), 0..16)
            ) {
                let b = MacroBits::from_words(&words, len);

                prop_assert_eq!(b.len(), len);
            }

            // word 数量保持不变
            #[test]
            fn word_count_preserved(
                len in 0usize..512,
                words in prop::collection::vec(any::<u64>(), 0..16)
            ) {
                let b = MacroBits::from_words(&words, len);

                prop_assert_eq!(b.data().len(), words.len());
            }

            // tail mask 必须正确
            #[test]
            fn tail_mask_correct(
                len in 0usize..512,
                words in prop::collection::vec(any::<u64>(), 1..16)
            ) {
                let b = MacroBits::from_words(&words, len);

                let rem = len % MBLH::WORD_BIT_WIDTH;

                let last = *b.data().last().unwrap();

                if rem == 0 {
                    prop_assert_eq!(last, words.last().copied().unwrap());
                } else {
                    let mask = (1u64 << rem) - 1;
                    prop_assert_eq!(last, words.last().unwrap() & mask);
                }
            }
        }
    }
}

impl MacroBits {
    #[inline]
    pub fn to_words(&self) -> &[u64] {
        &self.data
    }

    #[inline]
    pub fn to_words_vec(&self) -> Vec<u64> {
        self.data.to_vec()
    }

    #[inline]
    pub fn into_words(self) -> Box<[u64]> {
        self.data
    }

    #[inline]
    pub fn into_words_vec(self) -> Vec<u64> {
        self.data.into_vec()
    }
}
