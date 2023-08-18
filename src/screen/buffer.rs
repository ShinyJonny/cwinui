use crate::Pos;
use crate::style::{StyledStr, StyledChar, Style};
use crate::util::offset;

use super::{Cursor, InternalStyle};

/// The target buffer, passed to `Widget::render`.
#[derive(Debug)]
pub struct Buffer<'b> {
    width: u16,
    height: u16,
    chars: &'b mut Vec<char>,
    styles: &'b mut Vec<InternalStyle>,
    cursor: &'b mut Cursor,
}

// TODO: take a limiting area in all the methods.

impl<'b> Buffer<'b> {
    pub(super) fn from_raw_parts(
        width: u16,
        height: u16,
        chars: &'b mut Vec<char>,
        styles: &'b mut Vec<InternalStyle>,
        cursor: &'b mut Cursor,
    ) -> Self
    {
        Self {
            width,
            height,
            chars,
            styles,
            cursor,
        }
    }

    pub fn print<'s, T>(&mut self, x: u16, y: u16, text: T)
    where
        T: Into<StyledStr<'s>>
    {
        let x = x as usize;
        let y = y as usize;
        let text: StyledStr<'_> = text.into();

        let width = self.width as usize;
        let height = self.height as usize;

        if x >= width || y >= height {
            return;
        }

        // TODO: support printing with newlines (and other non-standard
        // whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        let char_count = text.content.chars().count();
        let print_len = if x + char_count > width {
            width - x
        } else {
            char_count
        };

        let mut chars = text.content.chars();
        for i in 0..print_len {
            self.chars[offset!(x + i, y, width)] = chars.next().unwrap();
        }

        let style = self.style_to_internal_with_fallback(x as u16, y as u16, text.style);
        for i in 0..print_len {
            self.styles[offset!(x + i, y, width)] = style;
        }
    }

    pub fn putc<T>(&mut self, x: u16, y: u16, c: T)
    where
        T: Into<StyledChar>
    {
        let c = c.into();

        if x >= self.width || y >= self.height {
            return;
        }

        let w = self.width as usize;
        let pos = offset!(x as usize, y as usize, w);
        self.chars[pos] = c.content;
        self.styles[pos] = self.style_to_internal_with_fallback(x, y, c.style);
    }

    pub fn hfill<T>(&mut self, x: u16, y: u16, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = x as usize;
        let y = y as usize;
        let c = c.into();

        let width = self.width as usize;
        let height = self.height as usize;

        if x >= width || y >= height {
            return;
        }

        let fill_len = if x + len > width { width - x } else { len };

        for i in 0..fill_len {
            self.chars[offset!(x + i, y, width)] = c.content;
        }

        let style
            = self.style_to_internal_with_fallback(x as u16, y as u16, c.style);
        for i in 0..fill_len {
            self.styles[offset!(x + i, y, width)] = style;
        }
    }

    pub fn vfill<T>(&mut self, x: u16, y: u16, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = x as usize;
        let y = y as usize;
        let c = c.into();

        let width = self.width as usize;
        let height = self.height as usize;

        if x >= width || y >= height {
            return;
        }

        let fill_len = if y + len > height
            { height - y }
            else { len };

        for i in 0..fill_len {
            self.chars[offset!(x, y + i, width)] = c.content;
        }

        let style
            = self.style_to_internal_with_fallback(x as u16, y as u16, c.style);
        for i in 0..fill_len {
            self.styles[offset!(x, y + i, width)] = style;
        }
    }

    pub fn clear(&mut self)
    {
        self.chars.fill('\0');
        self.styles.fill(InternalStyle::default());
    }

    pub fn show_cursor(&mut self)
    {
        self.cursor.hidden = false;
    }

    pub fn hide_cursor(&mut self)
    {
        self.cursor.hidden = true;
    }

    pub fn move_cursor(&mut self, pos: Pos)
    {
        if pos.x >= self.width || pos.y >= self.height {
            return;
        }

        self.cursor.x = pos.x;
        self.cursor.y = pos.y;
    }

    pub fn advance_cursor(&mut self, steps: i16)
    {
        if steps < 0 {
            if (-steps) as u16 > self.cursor.x {
                return;
            }
        } else if steps as u16 + self.cursor.x >= self.width {
            return;
        }

        self.cursor.x = (self.cursor.x as i16 + steps) as u16;
    }

    /// Converts `style` to `InternalStyle`
    ///
    /// If a component of `Style` is missing, the style at the offset just
    /// before the one specified by `x` and `y` is used; or `Default::default`.
    ///
    /// **NOTE**: doesn't check the bounds.
    #[inline]
    fn style_to_internal_with_fallback(
        &self,
        start_x: u16,
        start_y: u16,
        style: Style
    ) -> InternalStyle
    {
        let start_idx
            = offset!(start_x as usize, start_y as usize, self.width as usize);
        let fallb_style = if start_idx > 0
            { self.styles[start_idx - 1] }
            else { InternalStyle::default() };

        InternalStyle {
            fg_color: style.fg_color.unwrap_or(fallb_style.fg_color),
            bg_color: style.bg_color.unwrap_or(fallb_style.bg_color),
            text_style: style.text_style.unwrap_or(fallb_style.text_style),
        }
    }
}
