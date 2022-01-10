use crate::widget::{InnerWidget, Widget, InteractiveWidget, OutputWidget};
use crate::misc::PoisonError;
use crate::layout::{self, Aligned, Align};

extern crate termion;

use termion::event::{Event, Key};

const BLANK_CHAR: char = '_';
const INPUT_CAPACITY: usize = 1024;

pub struct Prompt {
    inner: InnerWidget,
    length: usize,
    output_ready: bool,
    output: String,
    cursor_pos: u32,
}

impl Prompt {
    pub fn new(y: u32, x: u32, length: usize) -> Self
    {
        let mut inner = InnerWidget::new(y, x, 1, length);
        inner.show_cursor();
        for i in 0..length {
            inner.putc(0, i as u32, BLANK_CHAR);
        }

        Self {
            inner,
            length,
            output_ready: false,
            output: String::with_capacity(INPUT_CAPACITY),
            cursor_pos: 0,
        }
    }
}

impl Widget for Prompt {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}

impl InteractiveWidget for Prompt {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Char('\n')) => {
                self.output_ready = true;
            },
            Event::Key(Key::Char(c)) => {
                if c.is_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                    if self.output.len() + 1 < self.length {
                        self.output.push(c);
                        self.inner.putc(0, self.cursor_pos, c);
                        self.inner.advance_cursor(1);
                        self.cursor_pos += 1;
                    } else {
                        self.output.push(c);
                        self.inner.print(0, 0, &self.output[self.output.len() + 1 - self.length..]);
                    }
                }
            },
            Event::Key(Key::Backspace) => {
                if !self.output.is_empty() {
                    if self.output.len() + 1 <= self.length {
                        self.output.pop();
                        self.inner.putc(0, self.cursor_pos - 1, BLANK_CHAR);
                        self.inner.advance_cursor(-1);
                        self.cursor_pos -= 1;
                    } else {
                        self.output.pop();
                        self.inner.print(0, 0, &self.output[self.output.len() + 1 - self.length..]);
                    }
                }
            },
            // TODO: arrow keys
            // TODO: Event::Key(Key::Delete) => {},
            _ => (),
        }
    }
}

impl OutputWidget<String> for Prompt {
    fn try_get_output(&mut self) -> Option<String>
    {
        if self.output_ready {
            let mut s = String::with_capacity(INPUT_CAPACITY);
            std::mem::swap(&mut s, &mut self.output);
            return Some(s);
        }
        None
    }

    fn get_output(self) -> Result<String, PoisonError<String>>
    {
        if self.output_ready {
            return Ok(self.output);
        }
        Err(PoisonError::new(self.output))
    }
}

impl Aligned for Prompt {
    fn inner_width(&self) -> usize
    {
        self.outer_width()
    }

    fn inner_height(&self) -> usize
    {
        self.outer_height()
    }

    fn inner_start_yx(&self) -> (u32, u32)
    {
        self.outer_start_yx()
    }

    fn outer_width(&self) -> usize
    {
        self.inner.borrow().width
    }

    fn outer_height(&self) -> usize
    {
        self.inner.borrow().height
    }

    fn outer_start_yx(&self) -> (u32, u32)
    {
        let inner = self.inner.borrow();
        (inner.start_y, inner.start_x)
    }

    fn centre(&self) -> (u32, u32)
    {
        let inner = self.inner.borrow();

        let (mut centre_y, mut centre_x) = (
            inner.start_y + inner.height as u32 / 2,
            inner.start_x + inner.width as u32 / 2
        );
        if centre_y > 0 {
            centre_y -= 1;
        }
        if centre_x > 0 {
            centre_x -= 1;
        }

        (centre_y, centre_x)
    }

    fn align_centres<T: Aligned>(&mut self, anchor: &T)
    {
        let (acy, acx) = anchor.centre();
        let (scy, scx) = self.centre();

        let acy = acy as i64;
        let acx = acx as i64;
        let scy = scy as i64;
        let scx = scx as i64;

        let mut inner = self.inner.borrow_mut();
        inner.start_y = (inner.start_y as i64 + (acy - scy)) as u32;
        inner.start_x = (inner.start_x as i64 + (acx - scx)) as u32;
    }

    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let mut inner = self.inner.borrow_mut();

        let (ay, ax) = anchor.inner_start_yx();
        let aheight = anchor.inner_height();
        let awidth = anchor.inner_width();
        let sheight = inner.height;
        let swidth = inner.width;

        let (new_y, new_x) = layout::align(
            a,
            sheight, swidth,
            ay, ax, aheight, awidth
        );

        inner.start_y = new_y;
        inner.start_x = new_x;
    }

    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let mut inner = self.inner.borrow_mut();

        let (ay, ax) = anchor.outer_start_yx();
        let aheight = anchor.outer_height();
        let awidth = anchor.outer_width();
        let sheight = inner.height;
        let swidth = inner.width;

        let (new_y, new_x) = layout::align(
            a,
            sheight, swidth,
            ay, ax, aheight, awidth
        );

        inner.start_y = new_y;
        inner.start_x = new_x;
    }
}
