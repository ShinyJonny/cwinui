use std::ops::{Range, RangeFrom, RangeTo};


pub trait SliceByChars<Idx> {
    fn slice_by_chars(&self, index: Idx) -> &str;
}

impl SliceByChars<Range<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: Range<usize>) -> &str
    {
        let start_bytes = bytes_until(index.start, self);
        let end_bytes = bytes_until(index.end, self);

        &self[start_bytes..end_bytes]
    }
}

impl SliceByChars<RangeFrom<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: RangeFrom<usize>) -> &str
    {
        let start_bytes = bytes_until(index.start, self);

        &self[start_bytes..]
    }
}

impl SliceByChars<RangeTo<usize>> for str {
    #[inline]
    fn slice_by_chars(&self, index: RangeTo<usize>) -> &str
    {
        let end_bytes = bytes_until(index.end, self);

        &self[..end_bytes]
    }
}


pub trait SliceByCharsMut<Idx> {
    fn slice_by_chars_mut(&mut self, index: Idx) -> &mut str;
}

impl SliceByCharsMut<Range<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: Range<usize>) -> &mut str
    {
        let start_bytes = bytes_until(index.start, self);
        let end_bytes = bytes_until(index.end, self);

        &mut self[start_bytes..end_bytes]
    }
}

impl SliceByCharsMut<RangeFrom<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: RangeFrom<usize>) -> &mut str
    {
        let start_bytes = bytes_until(index.start, self);

        &mut self[start_bytes..]
    }
}

impl SliceByCharsMut<RangeTo<usize>> for str {
    #[inline]
    fn slice_by_chars_mut(&mut self, index: RangeTo<usize>) -> &mut str
    {
        let end_bytes = bytes_until(index.end, self);

        &mut self[..end_bytes]
    }
}


// TODO: inclusive ranges.


#[inline]
fn bytes_until(until: usize, s: &str) -> usize
{
    let mut chars = s.chars();
    (0..until).fold(0, |acc, _| acc + chars.next().unwrap().len_utf8())
}
