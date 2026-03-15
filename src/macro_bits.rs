mod construction;
mod ops;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WideBits {
    len: usize,       // 比特长度
    data: Box<[u64]>, // 真正存储比特的堆内存
}

// basic accessors
impl WideBits {
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
    pub fn into_parts(self) -> (usize, Box<[u64]>) {
        (self.len, self.data)
    }
}
