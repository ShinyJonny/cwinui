use crate::render::Render;
use crate::{Pos, Area};
use crate::style::{AsStyledStr, Style, StyledChar};
use crate::util::offset;

/// Internals determining the state of the cursor.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Cursor {
    pub x: u16,
    pub y: u16,
    pub hidden: bool,
}

/// Versatile container-agnostic buffer that can be used for painting widgets.
#[derive(Debug)]
pub struct Buffer<'a> {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) chars: &'a mut [char],
    pub(crate) styles: &'a mut [Style],
    pub(crate) cursor: &'a mut Cursor,
}

impl<'a> Buffer<'a> {
    /// Creates a new `Buffer`.
    ///
    /// # Panics
    ///
    /// If the length of `chars` or `styles` is less than `width * height`.
    pub(crate) fn new(
        width: u16,
        height: u16,
        chars: &'a mut [char],
        styles: &'a mut [Style],
        cursor: &'a mut Cursor
    ) -> Self
    {
        assert!(chars.len() >= width as usize * height as usize);
        assert!(styles.len() >= width as usize * height as usize);

        Self {
            width,
            height,
            chars,
            styles,
            cursor,
        }
    }
}

impl Render for Buffer<'_> {
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
    fn set_str<S: AsStyledStr>(&mut self, pos: Pos, text: S)
    {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let w = self.width as usize;

        let text = text.as_styled_str();

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
    fn set_char<T>(&mut self, pos: Pos, c: T)
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
        *self.cursor = Cursor { x: 0, y: 0, hidden: true };
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
