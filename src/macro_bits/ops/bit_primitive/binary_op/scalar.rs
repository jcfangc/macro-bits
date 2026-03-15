use crate::{WBLH, WideBits};

impl WideBits {
    #[inline]
    fn binary_op_scalar_kernel<F>(lhs: &Self, rhs: &Self, scalar_op: F) -> (usize, Box<[u64]>)
    where
        F: Fn(u64, u64) -> u64,
    {
        let len = lhs.len.min(rhs.len);
        let word_len = WBLH::required_word_len(len);
        let mut data = Vec::with_capacity(word_len);

        for i in 0..word_len {
            data.push(scalar_op(lhs.data[i], rhs.data[i]));
        }

        let mut data = data.into_boxed_slice();
        WBLH::sanitize_last_word(&mut data, len);
        (len, data)
    }

    #[inline]
    fn binary_op_scalar<F>(&self, rhs: &Self, scalar_op: F) -> Self
    where
        F: Fn(u64, u64) -> u64,
    {
        let (len, data) = Self::binary_op_scalar_kernel(self, rhs, scalar_op);
        Self::new_unchecked(len, data)
    }

    #[inline]
    fn binary_op_assign_scalar<F>(&mut self, rhs: &Self, scalar_op: F)
    where
        F: Fn(u64, u64) -> u64,
    {
        let (len, data) = Self::binary_op_scalar_kernel(self, rhs, scalar_op);
        self.len = len;
        self.data = data;
    }
}

impl WideBits {
    #[inline]
    pub(super) fn and_scalar(&self, rhs: &Self) -> Self {
        self.binary_op_scalar(rhs, |x, y| x & y)
    }

    #[inline]
    pub(super) fn and_assign_scalar(&mut self, rhs: &Self) {
        self.binary_op_assign_scalar(rhs, |x, y| x & y)
    }

    #[inline]
    pub(super) fn or_scalar(&self, rhs: &Self) -> Self {
        self.binary_op_scalar(rhs, |x, y| x | y)
    }

    #[inline]
    pub(super) fn or_assign_scalar(&mut self, rhs: &Self) {
        self.binary_op_assign_scalar(rhs, |x, y| x | y)
    }

    #[inline]
    pub(super) fn xor_scalar(&self, rhs: &Self) -> Self {
        self.binary_op_scalar(rhs, |x, y| x ^ y)
    }

    #[inline]
    pub(super) fn xor_assign_scalar(&mut self, rhs: &Self) {
        self.binary_op_assign_scalar(rhs, |x, y| x ^ y)
    }

    #[inline]
    pub(super) fn andnot_scalar(&self, rhs: &Self) -> Self {
        self.binary_op_scalar(rhs, |x, y| x & !y)
    }

    #[inline]
    pub(super) fn andnot_assign_scalar(&mut self, rhs: &Self) {
        self.binary_op_assign_scalar(rhs, |x, y| x & !y)
    }
}
