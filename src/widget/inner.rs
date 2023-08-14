use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use crate::layout::Area;
use crate::util::offset;
use crate::style::{Style, StyledChar, StyledStr};

pub struct Cursor {
    pub y: u16,
    pub x: u16,
    pub hidden: bool,
}

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub style_buffer: Vec<Style>,
    pub cursor: Cursor,
    pub start_x: u16,
    pub start_y: u16,
    pub width: u16,
    pub height: u16,
    pub z_index: u16,
    pub hidden: bool,
    pub subwidgets: Vec<InnerWidget>,
}

pub struct InnerWidget(Rc<RefCell<InnerWidgetBody>>);

impl InnerWidget {
    pub fn new(area: Area) -> Self
    {
        let Area { x: start_x, y: start_y, width, height } = area;
        Self (
            Rc::new(RefCell::new(
                InnerWidgetBody {
                    buffer: vec!['\0'; (width * height) as usize],
                    style_buffer: vec![Style::default(); (width * height) as usize],
                    start_x,
                    start_y,
                    width,
                    height,
                    cursor: Cursor { y: 0, x: 0, hidden: true },
                    z_index: 1,
                    hidden: true,
                    subwidgets: Vec::new(),
                }
            ))
        )
    }

    pub fn share(&self) -> Self
    {
        InnerWidget(Rc::clone(&self))
    }

    pub fn add_subwidget(&mut self, sub: InnerWidget)
    {
        self.borrow_mut().subwidgets.push(sub);
    }

    pub fn print<'s, T>(&self, x: u16, y: u16, text: T)
    where
        T: Into<StyledStr<'s>>
    {
        let x = x as usize;
        let y = y as usize;
        let text = text.into();

        let mut body = self.borrow_mut();
        let width = body.width as usize;

        if x >= width || y >= width {
            return;
        }

        // TODO: support printing with newlines (and other non-standard whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        let text_chars = text.content.chars().count();
        let print_len = if x + text_chars > width {
            width - x
        } else {
            text_chars
        };

        let mut chars = text.content.chars();
        for i in 0..print_len {
            body.buffer[offset!(x + i, y, width)] = chars.next().unwrap();
        }

        for i in 0..print_len {
            body.style_buffer[offset!(x + i, y, width)] = text.style;
        }
    }

    pub fn putc<T>(&self, x: u16, y: u16, c: T)
    where
        T: Into<StyledChar>
    {
        let c = c.into();
        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        let w = body.width as usize;
        let pos = offset!(x as usize, y as usize, w);
        body.buffer[pos] = c.content;
        body.style_buffer[pos] = c.style;
    }

    pub fn hfill<T>(&self, x: u16, y: u16, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = x as usize;
        let y = y as usize;
        let c = c.into();

        let mut body = self.borrow_mut();

        let width = body.width as usize;

        if x >= width || y >= width {
            return;
        }

        let fill_len = if x + len > width { width - x } else { len };

        for i in 0..fill_len {
            body.buffer[offset!(x + i, y, width)] = c.content;
        }

        for i in 0..fill_len {
            body.style_buffer[offset!(x + i, y, width)] = c.style;
        }
    }

    pub fn vfill<T>(&self, x: u16, y: u16, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let x = x as usize;
        let y = y as usize;
        let c = c.into();

        let mut body = self.borrow_mut();

        let width = body.width as usize;
        let height = body.height as usize;

        if x >= width || y >= height {
            return;
        }

        let fill_len = if y + len > height { height - y } else { len };

        for i in 0..fill_len {
            body.buffer[offset!(x, y + i, width)] = c.content;
        }

        for i in 0..fill_len {
            body.style_buffer[offset!(x, y + i, width)] = c.style;
        }
    }

    #[inline]
    pub fn peekc(&self, x: u16, y: u16) -> StyledChar
    {
        let inner = self.borrow();
        let pos = offset!(x as usize, y as usize, inner.width as usize);

        StyledChar {
            content: inner.buffer[pos],
            style: inner.style_buffer[pos]
        }
    }

    pub fn clear(&self)
    {
        let mut inner = self.borrow_mut();

        // FIXME: optimise into one loop.
        for c in inner.buffer.iter_mut() {
            *c = '\0';
        }
        for s in inner.style_buffer.iter_mut() {
            *s = Style::default();
        }
    }

    pub fn show_cursor(&self)
    {
        self.borrow_mut().cursor.hidden = false;
    }

    pub fn hide_cursor(&self)
    {
        self.borrow_mut().cursor.hidden = true;
    }

    pub fn move_cursor(&self, x: u16, y: u16)
    {
        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        body.cursor.x = x;
        body.cursor.y = y;
    }

    pub fn advance_cursor(&self, steps: i16)
    {
        let mut body = self.borrow_mut();

        if steps < 0 {
            if (-steps) as u16 > body.cursor.x {
                return;
            }
        } else if steps as u16 + body.cursor.x >= body.width {
            return;
        }

        body.cursor.x = (body.cursor.x as i16 + steps) as u16;
    }

    /// Resizes the widget.
    /// This does not preserve the contents. Users should always treat it as though the contents
    /// become garbage.
    // TODO: create a struct for just the sizes, similar to how `Pos` is just
    // for the position.
    pub fn resize(&mut self, width: u16, height: u16)
    {
        let buf_size = width as usize * height as usize;
        let mut body = self.borrow_mut();

        body.width = width;
        body.height = height;
        body.buffer.resize(buf_size, '\0');
        body.style_buffer.resize(buf_size, Style::default());
    }
}

impl Deref for InnerWidget {
    type Target = Rc<RefCell<InnerWidgetBody>>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
