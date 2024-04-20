use bitflags::bitflags;

/// Styling data used to style text.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
pub struct Style {
    pub text_style: Option<TextStyle>,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Style {
    /// Resets the style.
    #[inline]
    pub fn clean(mut self) -> Self
    {
        self.text_style = Some(TextStyle::NORMAL);
        self.fg_color = Some(Color::Normal);
        self.bg_color = Some(Color::Normal);

        self
    }

    /// Adjusts the text style.
    #[inline]
    pub fn text_style(mut self, new_ts: TextStyle) -> Self
    {
        self.text_style = Some(new_ts);

        self
    }

    /// Adjusts the foreground color.
    #[inline]
    pub fn fg(mut self, color: Color) -> Self
    {
        self.fg_color = Some(color);

        self
    }

    /// Adjusts the background color.
    #[inline]
    pub fn bg(mut self, color: Color) -> Self
    {
        self.bg_color = Some(color);

        self
    }

    /// Overrides with values from `other` that aren't `None`.
    #[inline]
    pub fn merge(&self, other: Self) -> Self
    {
        let mut ret = self.clone();

        if other.text_style.is_some() {
            ret.text_style = other.text_style;
        }
        if other.fg_color.is_some() {
            ret.fg_color = other.fg_color;
        }
        if other.bg_color.is_some() {
            ret.bg_color = other.bg_color;
        }

        ret
    }
}

bitflags! {
    /// Used to define style special text styling in consoles, e.g. bold text,
    /// underlined text, blinking, etc.
    pub struct TextStyle: u8 {
        const NORMAL    = 0b00000000;
        const BOLD      = 0b00000001;
        const BLINK     = 0b00000010;
        const INVERT    = 0b00000100;
        const ITALIC    = 0b00001000;
        const UNDERLINE = 0b00010000;
    }
}

impl Default for TextStyle {
    fn default() -> Self
    {
        Self::NORMAL
    }
}

/// Colors supporting the standard 16 terminal colors, ANSI 256 colors and full
/// true colors (24-bit RGB).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, std::hash::Hash)]
pub enum Color {
    #[default]
    Normal,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
    Ansi(u8),
    // FIXME: flatten the tuple.
    Rgb(u8, u8, u8),
}

/// String slice with attached `Style`.
///
/// For owned version, see [StyledString].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyledStr<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s> StyledStr<'s> {
    // FIXME: properly implement `Borrow` and `ToOwned`.
    /// Converts to `StyledString`.
    #[inline]
    pub fn to_owned(&self) -> StyledString
    {
        StyledString::from(*self)
    }

    /// Slices the contained `str`, clones the [Style] and constructs a new
    /// `StyledStr`.
    #[inline]
    pub fn slice<T>(&self, index: T) -> Self
    where
        T: std::slice::SliceIndex<str>,
        T::Output: AsRef<str> + 's,
    {
        Self {
            content: self.content[index].as_ref(),
            style: self.style,
        }
    }
}

impl<'s, T> From<&'s T> for StyledStr<'s>
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

/// Owned version of [StyledStr].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledString {
    pub content: String,
    pub style: Style,
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

/// Char with attached style.
///
/// See also [StyledStr] and [StyledString].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyledChar {
    pub content: char,
    pub style: Style,
}

impl From<char> for StyledChar {
    fn from(c: char) -> Self
    {
        Self {
            content: c,
            style: Style::default(),
        }
    }
}

/// Structures that can be wrapped with [Style].
pub trait WithStyle<T>
{
    /// Converts `self` into `T` (styled type) and allows the style to be
    /// introspected and modified via `f`.
    fn with_style<F>(self, f: F) -> T
    where
        F: FnOnce(Style) -> Style;
    /// Converts `self` into `T` (styled type).
    fn styled(self) -> T;
}

impl<T> WithStyle<StyledChar> for T
where
    T: Into<StyledChar>
{
    #[inline]
    fn with_style<F>(self, f: F) -> StyledChar
    where
        F: FnOnce(Style) -> Style
    {
        let mut new = self.into();
        new.style = f(new.style);

        new
    }

    #[inline]
    fn styled(self) -> StyledChar
    {
        self.into()
    }
}

impl<'s, T> WithStyle<StyledStr<'s>> for T
where
    T: Into<StyledStr<'s>>
{
    #[inline]
    fn with_style<F>(self, f: F) -> StyledStr<'s>
    where
        F: FnOnce(Style) -> Style
    {
        let mut new = self.into();
        new.style = f(new.style);

        new
    }

    #[inline]
    fn styled(self) -> StyledStr<'s>
    {
        self.into()
    }
}
