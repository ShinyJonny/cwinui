use crate::layout::Justify;
use crate::{Pos, Area};
use crate::style::{StyledStr, StyledChar, Style, WithStyle};
use crate::util::offset;

use super::Cursor;

/// The target buffer, passed to `Widget::render`.
#[derive(Debug)]
pub struct Buffer<'b> {
    width: u16,
    height: u16,
    chars: &'b mut Vec<char>,
    styles: &'b mut Vec<Style>,
    cursor: &'b mut Cursor,
}

// TODO: take a limiting area in all the methods.

impl<'b> Buffer<'b> {
    pub(super) fn from_raw_parts(
        width: u16,
        height: u16,
        chars: &'b mut Vec<char>,
        styles: &'b mut Vec<Style>,
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

    pub fn printa<'s, S>(&mut self, x: u16, y: u16, text: S, area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        if !area.overlaps(self.area()) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_void() {
            return;
        }

        if !area.contains_pos(Pos { x, y }) {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        let area_right_end = area.x as usize + area.width as usize;

        let text: StyledStr<'_> = text.into();

        // TODO: support printing with newlines (and other non-standard
        // whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        // TODO: utf8 support.
        let text_len = text.content.len();
        let print_width = if x + text_len > area_right_end
            { area_right_end - x }
            else { text_len };

        let mut chars = text.content.chars();

        for i in 0..print_width {
            let offset = offset!(x + i, y, self.width as usize);

            self.chars[offset] = chars.next().unwrap();
            self.set_cell_style(offset, text.style)
        }
    }

    pub fn print<'s, T>(&mut self, x: u16, y: u16, text: T)
    where
        T: Into<StyledStr<'s>>
    {
        self.printa(x, y, text, self.area());
    }

    pub fn printj<'s, S>(&mut self, text: S, j: Justify ,area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_void() {
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
        self.set_cell_style(idx, c.style);
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
            let offset = offset!(x + i, y, width);

            self.chars[offset] = c.content;
            self.set_cell_style(offset, c.style);
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
            let offset = offset!(x, y + i, width);

            self.chars[offset] = c.content;
            self.set_cell_style(offset, c.style);
        }
    }

    pub fn clear(&mut self)
    {
        self.chars.fill(' ');
        self.styles.fill(Style::default().clean());
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

    #[inline]
    fn set_cell_style(&mut self, offset: usize, style: Style)
    {
        let cell = &mut self.styles[offset];

        let Style { text_style, fg_color, bg_color } = style;

        if let Some(ts) = text_style {
            cell.text_style = Some(ts);
        }
        if let Some(fg) = fg_color {
            cell.fg_color = Some(fg);
        }
        if let Some(bg) = bg_color {
            cell.bg_color = Some(bg);
        }
    }
}
