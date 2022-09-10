use crate::style::Style;

// TODO: implement slicing.

#[derive(PartialEq, Eq)]
pub struct StyledText<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s> From<&'s str> for StyledText<'s> {
    fn from(s: &'s str) -> Self
    {
        Self {
            content: s,
            style: Style::default(),
        }
    }
}

#[derive(PartialEq, Eq)]
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
