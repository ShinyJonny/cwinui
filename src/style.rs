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

pub enum Style {
    Reset,
    Bold,
    Blink,
    Invert,
    Italic,
    Underline,
    NotBold,
    NotBlink,
    NotInvert,
    NotItalic,
    NotUnderline,
    ResetColor,
    Color16(Color16),
    Color256(u8),
    ColorTrue(u8, u8, u8),
}

pub struct StyleMarker {
    pub style: Style,
    pub offset: usize,
}
