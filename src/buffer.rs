use crate::paint::Paint;
use crate::{Pos, Area};
use crate::style::{StyledStr, StyledChar, Style};
use crate::util::offset;

#[derive(Debug)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub hidden: bool,
}

/// Versatile buffer that can be used for painting widgets.
#[derive(Debug)]
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    pub chars: Vec<char>,
    pub styles: Vec<Style>,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self
    {
        let buf_size = width as usize * height as usize;

        Self {
            width,
            height,
            chars: vec![' '; buf_size],
            styles: vec![Style::default().clean(); buf_size],
            cursor: Cursor { y: 0, x: 0, hidden: true },
        }
    }

}

impl Paint for Buffer {
    fn area(&self) -> Area
    {
        Area {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    /// Prints `text` relative to the specified `area`, potentially truncating
    /// its contents.
    fn print<'s, S>(&mut self, pos: Pos, text: S, area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        let Pos {x, y} = pos;

        if !area.overlaps(self.area()) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_void() {
            return;
        }

        if x >= area.width || y >= area.height {
            return;
        }

        let abs_x = (area.x + x) as usize;
        let abs_y = (area.y + y) as usize;

        let area_right_end = (area.x + area.width) as usize;

        let text: StyledStr<'_> = text.into();

        // TODO: support printing with newlines (and other non-standard
        // whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        // TODO: utf8 support.
        let text_len = text.content.len();
        let print_width = if abs_x + text_len > area_right_end
            { area_right_end - abs_x }
            else { text_len };

        let mut chars = text.content.chars();

        for i in 0..print_width {
            let offset = offset!(abs_x + i, abs_y, self.width as usize);

            self.chars[offset] = chars.next().unwrap();
            let style = &mut self.styles[offset];
            *style = style.merge(text.style);
        }
    }

    fn putc<T>(&mut self, pos: Pos, c: T)
    where
        T: Into<StyledChar>
    {
        let Pos {x, y} = pos;
        if x >= self.width || y >= self.height {
            return;
        }

        let c = c.into();

        let w = self.width as usize;
        let idx = offset!(x as usize, y as usize, w);
        self.chars[idx] = c.content;
        let style = &mut self.styles[idx];
        *style = style.merge(c.style);
    }

    fn hfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = pos.x as usize;
        let y = pos.y as usize;
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
            let style = &mut self.styles[offset];
            *style = style.merge(c.style);
        }
    }

    fn vfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = pos.x as usize;
        let y = pos.y as usize;
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
            let style = &mut self.styles[offset];
            *style = style.merge(c.style);
        }
    }

    fn clear(&mut self)
    {
        self.chars.fill(' ');
        self.styles.fill(Style::default());
        self.cursor = Cursor { x: 0, y: 0, hidden: true };
    }

    fn show_cursor(&mut self)
    {
        self.cursor.hidden = false;
    }

    fn hide_cursor(&mut self)
    {
        self.cursor.hidden = true;
    }

    fn move_cursor(&mut self, pos: Pos)
    {
        if pos.x >= self.width || pos.y >= self.height {
            return;
        }

        self.cursor.x = pos.x;
        self.cursor.y = pos.y;
    }
}
