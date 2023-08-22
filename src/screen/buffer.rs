use crate::layout::Justify;
use crate::{Pos, Area};
use crate::style::{StyledStr, StyledChar, Style, WithStyle};
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

    #[inline]
    pub fn area(&self) -> Area
    {
        Area {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    pub fn print<'s, T>(&mut self, x: u16, y: u16, text: T)
    where
        T: Into<StyledStr<'s>>
    {
        let x = x as usize;
        let y = y as usize;

        let width = self.width as usize;
        let height = self.height as usize;

        if x >= width || y >= height {
            return;
        }

        let text: StyledStr<'_> = text.into();

        // TODO: support printing with newlines (and other non-standard
        // whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        // TODO: utf8 support.
        let text_len = text.content.len();
        let print_width = if x + text_len > width
            { width - x }
            else { text_len };

        let mut chars = text.content.chars();
        for i in 0..print_width {
            self.chars[offset!(x + i, y, width)] = chars.next().unwrap();
        }

        let style = self.style_to_internal_with_fallback(x as u16, y as u16, text.style);
        for i in 0..print_width {
            self.styles[offset!(x + i, y, width)] = style;
        }
    }

    pub fn printj<'s, S>(&mut self, text: S, j: Justify ,area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.width == 0 || area.height == 0 {
            return;
        }

        let text: StyledStr = text.into();
        // TODO: utf8 support.
        let text_width = text.content.len();
        // TODO: implement direct slicing of `StyledStr`.
        let text
            = text.content[..std::cmp::min(text_width, area.width as usize)]
                .with_style(|_| text.style);

        let Pos { x: rel_x, y: rel_y } = match j {
            Justify::Left(y) => Pos {
                x: 0,
                y
            },
            Justify::HCentre(y) => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y,
            },
            Justify::Right(y) => Pos {
                x: (area.width as usize - text_width) as u16,
                y,
            },
            Justify::Top(x) => Pos {
                x,
                y: 0,
            },
            Justify::VCentre(x) => Pos {
                x,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Bottom(x) => Pos {
                x,
                y: area.height.saturating_sub(1),
            },
            Justify::TopLeft => Pos { x: 0, y: 0 },
            Justify::TopCentre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: 0,
            },
            Justify::TopRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: 0,
            },
            Justify::CentreLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Centre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::CentreRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::BottomLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomCentre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1),
            },
        };

        self.print(area.x + rel_x, area.y + rel_y, text);
    }

    pub fn putc<T>(&mut self, x: u16, y: u16, c: T)
    where
        T: Into<StyledChar>
    {
        if x >= self.width || y >= self.height {
            return;
        }

        let c = c.into();

        let w = self.width as usize;
        let idx = offset!(x as usize, y as usize, w);
        self.chars[idx] = c.content;
        self.styles[idx] = self.style_to_internal_with_fallback(x, y, c.style);
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
