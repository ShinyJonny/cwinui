use crate::style::Style;

// TODO: implement slicing.

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StyledText<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s, T> From<&'s T> for StyledText<'s>
where
    T: AsRef<str> + ?Sized
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

pub trait IntoStyledText<'s>
{
    fn styled<F>(self, f: F) -> StyledText<'s>
    where
        F: FnOnce(Style) -> Style;
}

impl<'s, T> IntoStyledText<'s> for T
where
    T: Into<StyledText<'s>>
{
    fn styled<F>(self, f: F) -> StyledText<'s>
    where
        F: FnOnce(Style) -> Style
    {
        let mut new = self.into();
        new.style = f(new.style);

        new
    }
}
