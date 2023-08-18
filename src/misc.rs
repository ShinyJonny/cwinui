use std::ops::{Range, RangeFrom, RangeTo};

pub trait SliceByChars<Idx> {
    fn slice_by_chars(&self, index: Idx) -> &str;
}

pub trait SliceByCharsMut<Idx> {
    fn slice_by_chars_mut(&mut self, index: Idx) -> &mut str;
}

#[inline]
fn bytes_until(until: usize, s: &str) -> usize
{
    let mut chars = s.chars();
    (0..until).fold(0, |acc, _| acc + chars.next().unwrap().len_utf8())
}

impl SliceByChars<Range<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: Range<usize>) -> &str
    {
        let s = self.as_ref();

        let start_bytes = bytes_until(index.start, s);
        let end_bytes = bytes_until(index.end, s);

        &s[start_bytes..end_bytes]
    }
}

impl SliceByCharsMut<Range<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: Range<usize>) -> &mut str
    {
        let s = self.as_mut();

        let start_bytes = bytes_until(index.start, s);
        let end_bytes = bytes_until(index.end, s);

        &mut s[start_bytes..end_bytes]
    }
}

impl SliceByChars<RangeFrom<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: RangeFrom<usize>) -> &str
    {
        let s = self.as_ref();

        let start_bytes = bytes_until(index.start, s);

        &s[start_bytes..]
    }
}

impl SliceByCharsMut<RangeFrom<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: RangeFrom<usize>) -> &mut str
    {
        let s = self.as_mut();

        let start_bytes = bytes_until(index.start, s);

        &mut s[start_bytes..]
    }
}

impl SliceByChars<RangeTo<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: RangeTo<usize>) -> &str
    {
        let s = self.as_ref();

        let end_bytes = bytes_until(index.end, s);

        &s[..end_bytes]
    }
}

impl SliceByCharsMut<RangeTo<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: RangeTo<usize>) -> &mut str
    {
        let s = self.as_mut();

        let end_bytes = bytes_until(index.end, s);

        &mut s[..end_bytes]
    }
}

// TODO: inclusive ranges.
