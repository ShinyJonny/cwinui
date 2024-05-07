use crate::widget::Paint;
use crate::{Pos, Area};
use crate::style::{StyledStr, StyledChar, Style};
use crate::util::offset;

/// Internals determining the state of the cursor.
#[derive(Debug, Clone)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub hidden: bool,
}

// TODO: resizing.

// FIXME: most of the fields cannot remain public.

/// Versatile buffer that can be used for painting widgets.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub cursor: Cursor,
    pub width: u16,
    pub height: u16,
    pub chars: Vec<char>,
    pub styles: Vec<Style>,
}

impl Buffer {
    /// Creates a new `Buffer`.
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
    #[inline]
    fn area(&self) -> Area
    {
        Area {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    fn paint_str<'s, S>(&mut self, pos: Pos, text: S)
    where
        S: Into<StyledStr<'s>>
    {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let w = self.width as usize;

        let text: StyledStr<'_> = text.into();

        // TODO: support printing with newlines (and other non-standard
        // whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        // TODO: utf8 support.

        let mut chars = text.content.chars();

        for i in 0..text.content.len() {
            let offset = offset!(x + i, y, w);

            self.chars[offset] = chars.next().unwrap();
            let style = &mut self.styles[offset];
            *style = style.merge(text.style);
        }
    }

    #[inline]
    fn paint_char<T>(&mut self, pos: Pos, c: T)
    where
        T: Into<StyledChar>
    {
        let c = c.into();

        let idx = offset!(pos.x as usize, pos.y as usize, self.width as usize);
        self.chars[idx] = c.content;
        let style = &mut self.styles[idx];
        *style = style.merge(c.style);
    }

    #[inline]
    fn clear(&mut self)
    {
        self.chars.fill(' ');
        self.styles.fill(Style::default());
        self.cursor = Cursor { x: 0, y: 0, hidden: true };
    }

    #[inline]
    fn show_cursor(&mut self)
    {
        self.cursor.hidden = false;
    }

    #[inline]
    fn hide_cursor(&mut self)
    {
        self.cursor.hidden = true;
    }

    #[inline]
    fn move_cursor(&mut self, pos: Pos)
    {
        if pos.x >= self.width || pos.y >= self.height {
            return;
        }

        self.cursor.x = pos.x;
        self.cursor.y = pos.y;
    }
}
