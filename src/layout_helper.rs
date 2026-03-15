/// Macro Bits Layout Helper
pub(crate) struct MBLH;

impl MBLH {
    pub(crate) const WORD_BIT_WIDTH: usize = 64;

    #[inline]
    pub(crate) const fn word_len(len: usize) -> usize {
        len.div_ceil(Self::WORD_BIT_WIDTH)
    }

    #[inline]
    pub(crate) const fn tail_mask(rem: usize) -> u64 {
        if rem == 0 {
            u64::MAX
        } else {
            (1u64 << rem) - 1
        }
    }

    #[inline]
    pub(crate) const fn sanitize_last(data: &mut [u64], len: usize) {
        if let Some(last) = data.last_mut() {
            let rem = len % 64;
            *last &= Self::tail_mask(rem);
        }
    }
}
