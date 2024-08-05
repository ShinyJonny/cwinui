use crate::style::{AsStyledStr, Style, StyledStr};


/// Owned version of [`StyledStr`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledString {
    pub content: String,
    pub style: Style,
}

impl AsStyledStr for &StyledString {
    fn as_styled_str(&self) -> StyledStr
    {
        StyledStr {
            content: self.content.as_str(),
            style: self.style,
        }
    }
}

impl <'s, T> From<T> for StyledString
where
    T: Into<StyledStr<'s>>
{
    fn from(t: T) -> Self
    {
        let t: StyledStr = t.into();

        Self {
            content: String::from(t.content),
            style: t.style,
        }
    }
}


impl<'s> StyledStr<'s> {
    // FIXME: properly implement `Borrow` and `ToOwned`.
    /// Converts to `StyledString`.
    #[inline]
    pub fn to_owned(&self) -> StyledString
    {
        StyledString::from(*self)
    }
}

impl<'s> From<&'s StyledString> for StyledStr<'s>
{
    fn from(s: &'s StyledString) -> Self
    {
        Self {
            content: s.content.as_str(),
            style: s.style,
        }
    }
}
