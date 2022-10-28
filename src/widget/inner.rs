use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

use crate::pos;
use crate::style::{Style, StyledChar, StyledText};

pub struct Cursor {
    pub y: u32,
    pub x: u32,
    pub hidden: bool,
}

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub style_buffer: Vec<Style>,
    pub cursor: Cursor,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub z_index: u32,
    pub hidden: bool,
    pub subwidgets: Vec<InnerWidget>,
}

pub struct InnerWidget(Rc<RefCell<InnerWidgetBody>>);

impl InnerWidget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self (
            Rc::new(RefCell::new(
                InnerWidgetBody {
                    buffer: vec!['\0'; width * height],
                    style_buffer: vec![Style::default(); width * height],
                    start_y,
                    start_x,
                    height,
                    width,
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

    pub fn print<'s, T>(&self, y: u32, x: u32, text: T)
    where
        T: Into<StyledText<'s>>
    {
        let y = y as usize;
        let x = x as usize;
        let text = text.into();

        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        // TODO: support printing with newlines (and other non-standard whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        let mut print_len = text.content.chars().count();
        if x + print_len > body.width {
            print_len = body.width - x;
        }

        let w = body.width;
        let mut chars = text.content.chars();
        for i in 0..print_len {
            body.buffer[pos![w, y, x + i]] = chars.next().unwrap();
        }

        for i in 0..print_len {
            body.style_buffer[pos![w, y, x + i]] = text.style;
        }
    }

    pub fn putc<T>(&self, y: u32, x: u32, c: T)
    where
        T: Into<StyledChar>
    {
        let c = c.into();
        let mut body = self.borrow_mut();

        if x as usize >= body.width || y as usize >= body.height {
            return;
        }

        let w = body.width;
        let pos = pos![w, y as usize, x as usize];
        body.buffer[pos] = c.content;
        body.style_buffer[pos] = c.style;
    }

    pub fn hfill<T>(&self, y: u32, x: u32, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let y = y as usize;
        let x = x as usize;
        let c = c.into();

        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        let mut fill_len = len;
        if x + fill_len > body.width {
            fill_len = body.width - x;
        }

        let w = body.width;
        for i in 0..fill_len {
            body.buffer[pos![w, y, x + i]] = c.content;
        }

        for i in 0..fill_len {
            body.style_buffer[pos![w, y, x + i]] = c.style;
        }
    }

    pub fn vfill<T>(&self, y: u32, x: u32, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let y = y as usize;
        let x = x as usize;
        let c = c.into();

        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        let mut fill_len = len;
        if y + fill_len > body.width {
            fill_len = body.height - y;
        }

        let w = body.width;
        for i in 0..fill_len {
            body.buffer[pos![w, y + i, x]] = c.content;
        }

        for i in 0..fill_len {
            body.style_buffer[pos![w, y + i, x]] = c.style;
        }
    }

    #[inline]
    pub fn peekc(&self, y: u32, x: u32) -> StyledChar
    {
        let inner = self.borrow();
        let pos = pos![inner.width, y as usize, x as usize];

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

    pub fn move_cursor(&self, y: u32, x: u32)
    {
        let mut body = self.borrow_mut();

        if y as usize >= body.height || x as usize >= body.width {
            return;
        }

        body.cursor.y = y;
        body.cursor.x = x;
    }

    pub fn advance_cursor(&self, steps: i32)
    {
        let mut body = self.borrow_mut();

        if steps < 0 {
            if (-steps) as u32 > body.cursor.x {
                return;
            }
        } else if steps as u32 + body.cursor.x >= body.width as u32 {
            return;
        }

        body.cursor.x = (body.cursor.x as i32 + steps) as u32;
    }

    /// Resizes the widget.
    /// This does not preserve the contents. Users should always treat it as though the contents
    /// become garbage.
    pub fn resize(&mut self, height: usize, width: usize)
    {
        let buf_size = height * width;
        let mut body = self.borrow_mut();

        body.height = height;
        body.width = width;
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
