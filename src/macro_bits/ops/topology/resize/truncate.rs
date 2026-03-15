use crate::{WBLH, WideBits};

impl WideBits {
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len >= self.len {
            return;
        }

        self.len = new_len;
        let required = WBLH::required_word_len(new_len);
        self.data = self.data[..required].into();

        WBLH::sanitize_last_word(&mut self.data, new_len);
    }
}
