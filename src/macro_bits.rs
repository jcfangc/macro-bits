mod construction;
mod ops;

pub struct MacroBits {
    len: usize,       // 比特长度
    data: Box<[u64]>, // 真正存储比特的堆内存
}

// basic accessors
impl MacroBits {
    #[inline]
    pub(crate) const fn new_unchecked(len: usize, data: Box<[u64]>) -> Self {
        Self { len, data }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn data(&self) -> &[u64] {
        &self.data
    }

    #[inline]
    pub(crate) const fn len_mut(&mut self) -> &mut usize {
        &mut self.len
    }

    #[inline]
    pub(crate) const fn data_mut(&mut self) -> &mut [u64] {
        &mut self.data
    }

    #[inline]
    pub fn into_parts(self) -> (usize, Box<[u64]>) {
        (self.len, self.data)
    }
}
