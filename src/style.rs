use bitflags::bitflags;

/// Styling data used to style text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
pub struct Style {
    pub text_style: Option<TextStyle>,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Default for Style {
    #[inline]
    fn default() -> Self
    {
        Self::default()
    }
}

impl Style {
    /// Const version of `Default::default`.
    #[inline]
    pub const fn default() -> Self
    {
        Self {
            text_style: None,
            fg_color: None,
            bg_color: None,
        }
    }

    /// Resets the style.
    #[inline]
    pub const fn clean(mut self) -> Self
    {
        self.text_style = Some(TextStyle::NORMAL);
        self.fg_color = Some(Color::Normal);
        self.bg_color = Some(Color::Normal);

        self
    }

    /// Adjusts the text style.
    #[inline]
    pub const fn text_style(mut self, new_ts: TextStyle) -> Self
    {
        self.text_style = Some(new_ts);

        self
    }

    /// Adjusts the foreground color.
    #[inline]
    pub const fn fg(mut self, color: Color) -> Self
    {
        self.fg_color = Some(color);

        self
    }

    /// Adjusts the background color.
    #[inline]
    pub const fn bg(mut self, color: Color) -> Self
    {
        self.bg_color = Some(color);

        self
    }

    /// Overrides with values from `other` that aren't `None`.
    #[inline]
    pub const fn merge(&self, other: Self) -> Self
    {
        let mut ret = *self;

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
    #[derive(Default)]
    pub struct TextStyle: u8 {
        const NORMAL    = 0b00000000;
        const BOLD      = 0b00000001;
        const BLINK     = 0b00000010;
        const INVERT    = 0b00000100;
        const ITALIC    = 0b00001000;
        const UNDERLINE = 0b00010000;
    }
}

/// Colors, supporting the standard 16 terminal colors, ANSI 256 colors and true
/// colors (24-bit RGB).
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
    Rgb(u8, u8, u8),
}

/// `&str` with attached `Style`.
///
/// For owned version, see [`StyledString`](crate::alloc::string::StyledString).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyledStr<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s> StyledStr<'s> {
    /// Slices the contained `str`, clones the [`Style`] and constructs a new
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

/// Style-enhanced `AsRef<str>`.
pub trait AsStyledStr {
    fn as_styled_str(&self) -> StyledStr;
}

impl<T> AsStyledStr for T
where
    T: AsRef<str>
{
    fn as_styled_str(&self) -> StyledStr
    {
        StyledStr {
            content: self.as_ref(),
            style: Style::default(),
        }
    }
}

impl<'a> AsStyledStr for StyledStr<'a> {
    fn as_styled_str(&self) -> StyledStr
    {
        *self
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

impl<'s, T> From<&'s mut T> for StyledStr<'s>
where
    T: AsMut<str> + ?Sized
{
    fn from(s: &'s mut T) -> Self
    {
        Self {
            content: s.as_mut(),
            style: Style::default(),
        }
    }
}

/// Char with attached style.
///
/// See also [`StyledStr`] and
/// [`StyledString`](crate::alloc::string::StyledString).
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

/// Structures that can be wrapped with [`Style`].
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
