use crate::{WBLH, WideBits};

impl WideBits {
    #[inline]
    pub fn extend(&mut self, new_len: usize) {
        if new_len <= self.len {
            return;
        }

        let required = WBLH::required_word_len(new_len);

        if required > self.data.len() {
            let mut v = std::mem::take(&mut self.data).into_vec();
            v.resize(required, 0);
            self.data = v.into_boxed_slice();
        }

        self.len = new_len;
    }
}
