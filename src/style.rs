use bitflags::bitflags;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub text_style: Option<TextStyle>,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self
    {
        Self {
            text_style: None,
            fg_color: None,
            bg_color: None,
        }
    }
}

impl Style {
    pub fn clean(mut self) -> Self
    {
        self.text_style = Some(TextStyle::NORMAL);
        self.fg_color = Some(Color::Normal);
        self.bg_color = Some(Color::Normal);

        self
    }

    pub fn text_style(mut self, new_ts: TextStyle) -> Self
    {
        self.text_style = Some(new_ts);

        self
    }

    pub fn fg_color(mut self, color: Color) -> Self
    {
        self.fg_color = Some(color);

        self
    }

    pub fn bg_color(mut self, color: Color) -> Self
    {
        self.bg_color = Some(color);

        self
    }
}

bitflags! {
    pub struct TextStyle: u8 {
        const NORMAL    = 0b00000000;
        const BOLD      = 0b00000001;
        const BLINK     = 0b00000010;
        const INVERT    = 0b00000100;
        const ITALIC    = 0b00001000;
        const UNDERLINE = 0b00010000;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
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
    Rgb((u8, u8, u8)),
}

// TODO: implement slicing.

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StyledStr<'s> {
    pub content: &'s str,
    pub style: Style,
}

impl<'s> StyledStr<'s> {
    pub fn to_owned(&self) -> StyledString
    {
        StyledString::from(*self)
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub trait WithStyle<T>
{
    fn with_style<F>(self, f: F) -> T
    where
        F: FnOnce(Style) -> Style;
    fn styled(self) -> T;
}

impl<T> WithStyle<StyledChar> for T
where
    T: Into<StyledChar>
{
    fn with_style<F>(self, f: F) -> StyledChar
    where
        F: FnOnce(Style) -> Style
    {
        let mut new = self.into();
        new.style = f(new.style);

        new
    }

    fn styled(self) -> StyledChar
    {
        self.into()
    }
}

impl<'s, T> WithStyle<StyledStr<'s>> for T
where
    T: Into<StyledStr<'s>>
{
    fn with_style<F>(self, f: F) -> StyledStr<'s>
    where
        F: FnOnce(Style) -> Style
    {
        let mut new = self.into();
        new.style = f(new.style);

        new
    }

    fn styled(self) -> StyledStr<'s>
    {
        self.into()
    }
}
