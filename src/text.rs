use crate::style::Style;

// TODO: implement slicing.

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StyledText<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s, T> From<&'s T> for StyledText<'s>
where
    T: AsRef<str>
{
    fn from(s: &'s T) -> Self
    {
        Self {
            content: s.as_ref(),
            style: Style::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StyledChar {
    pub c: char,
    pub style: Style,
}

impl From<char> for StyledChar {
    fn from(c: char) -> Self
    {
        Self {
            c,
            style: Style::default(),
        }
    }
}
