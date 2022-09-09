use bitflags::bitflags;

type TextStyleType = u8;

#[derive(Clone, Copy)]
pub struct Style {
    pub text_style: Option<TextStyleType>,
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
    pub fn text_style(mut self, new_ts: u8) -> Self
    {
        if let Some(text_style) = self.text_style.as_mut() {
            *text_style |= new_ts;
        } else {
            self.text_style = Some(new_ts);
        }

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
    pub struct TextStyle: TextStyleType {
        const BOLD      = 0b00000001;
        const BLINK     = 0b00000010;
        const INVERT    = 0b00000100;
        const ITALIC    = 0b00001000;
        const UNDERLINE = 0b00010000;
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    C16(Color16),
    C256(u8),
    CTrue(u8, u8, u8),
}

#[derive(Clone, Copy)]
pub enum Color16 {
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
}
