use crate::{WBLH, WideBits, macro_bits::construction::ConstructionError};

impl WideBits {
    #[inline]
    pub fn try_from_words(words: &[u64], len: usize) -> Result<Self, ConstructionError> {
        let required_word_len = WBLH::required_word_len(len);
        if words.len() < required_word_len {
            return Err(ConstructionError::InsufficientWords {
                required: required_word_len,
                provided: words.len(),
            });
        }
        let mut data = words[..required_word_len].to_vec().into_boxed_slice();
        WBLH::sanitize_last_word(&mut data, len);
        Ok(Self::new_unchecked(len, data))
    }

    #[inline]
    pub fn try_from_words_boxed(
        mut data: Box<[u64]>,
        len: usize,
    ) -> Result<Self, ConstructionError> {
        let required_word_len = WBLH::required_word_len(len);
        if data.len() < required_word_len {
            return Err(ConstructionError::InsufficientWords {
                required: required_word_len,
                provided: data.len(),
            });
        }
        data = data[..required_word_len].into();
        WBLH::sanitize_last_word(&mut data, len);
        Ok(Self::new_unchecked(len, data))
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

            let b = WideBits::try_from_words(&words, 0).unwrap();

            assert_eq!(b.len(), 0);
            assert!(b.data().is_empty());
        }

        #[test]
        fn exact_word_boundary() {
            let words = [u64::MAX, u64::MAX];

            let b = WideBits::try_from_words(&words, 128).unwrap();

            assert_eq!(b.len(), 128);
            assert_eq!(b.data(), &words);
        }

        #[test]
        fn tail_mask_applied() {
            let words = [u64::MAX];

            let b = WideBits::try_from_words(&words, 10).unwrap();

            let expected = (1u64 << 10) - 1;

            assert_eq!(b.len(), 10);
            assert_eq!(b.data()[0], expected);
        }

        #[test]
        fn multi_word_tail_mask() {
            let words = [u64::MAX, u64::MAX];

            let b = WideBits::try_from_words(&words, 70).unwrap();

            assert_eq!(b.data()[0], u64::MAX);

            let rem = 70 % 64;
            let mask = (1u64 << rem) - 1;

            assert_eq!(b.data()[1], mask);
        }

        #[test]
        fn insufficient_words_error() {
            let words = [0u64];

            let err = WideBits::try_from_words(&words, 130).unwrap_err();

            assert_eq!(
                err,
                ConstructionError::InsufficientWords {
                    required: 3,
                    provided: 1
                }
            );
        }
    }

    mod prop_tests {
        use super::*;

        proptest! {

            #[test]
            fn len_preserved(
                len in 0usize..512,
                words in prop::collection::vec(any::<u64>(), 0..16)
            ) {

                let required = WBLH::required_word_len(len);

                if words.len() < required {
                    prop_assert!(WideBits::try_from_words(&words, len).is_err());
                } else {
                    let b = WideBits::try_from_words(&words, len).unwrap();
                    prop_assert_eq!(b.len(), len);
                }
            }

            #[test]
            fn word_len_correct(
                len in 0usize..512,
                words in prop::collection::vec(any::<u64>(), 0..16)
            ) {

                let required = WBLH::required_word_len(len);

                if words.len() >= required {
                    let b = WideBits::try_from_words(&words, len).unwrap();

                    prop_assert_eq!(b.data().len(), required);
                }
            }

            #[test]
            fn tail_mask_correct(
                len in 1usize..512,
                words in prop::collection::vec(any::<u64>(), 1..16)
            ) {

                let required = WBLH::required_word_len(len);

                if words.len() >= required {

                    let b = WideBits::try_from_words(&words, len).unwrap();

                    let rem = len % WBLH::WORD_BIT_WIDTH;

                    let last = *b.data().last().unwrap();

                    if rem == 0 {
                        prop_assert_eq!(last, words[required-1]);
                    } else {
                        let mask = (1u64 << rem) - 1;
                        prop_assert_eq!(last, words[required-1] & mask);
                    }
                }
            }
        }
    }
}

impl WideBits {
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
