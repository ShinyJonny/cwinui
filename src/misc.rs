pub trait SliceInChars {
    fn slice_in_chars(&self, start: usize, end: usize) -> &str;
    fn slice_in_chars_mut(&mut self, start: usize, end: usize) -> &mut str;
}

impl SliceInChars for str {
    fn slice_in_chars(&self, start: usize, end: usize) -> &str
    {
        let mut chars = self.chars();

        let mut start_bytes = 0;
        for _i in 0..start {
            start_bytes += chars.next().unwrap().len_utf8();
        }

        chars = self.chars();
        let mut end_bytes = 0;
        for _i in 0..end {
            end_bytes += chars.next().unwrap().len_utf8();
        }

        &self[start_bytes..end_bytes]
    }

    fn slice_in_chars_mut(&mut self, start: usize, end: usize) -> &mut str
    {
        let mut chars = self.chars();

        let mut start_bytes = 0;
        for _i in 0..start {
            start_bytes += chars.next().unwrap().len_utf8();
        }

        chars = self.chars();
        let mut end_bytes = 0;
        for _i in 0..end {
            end_bytes += chars.next().unwrap().len_utf8();
        }

        &mut self[start_bytes..end_bytes]
    }
}
