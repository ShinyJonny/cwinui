use termion::event::{Event, Key};

use super::{
    InnerWidget,
    Widget,
    InteractiveWidget,
    OutputWidget,
    PoisonError,
};
use crate::misc::SliceInChars;
use crate::layout::{
    self,
    Aligned,
    Alignable,
    Align,
};

const BLANK_CHAR: char = ' ';
const INACTIVE_BLANK_CHAR: char = ' ';
const INPUT_CAPACITY: usize = 2048;

pub struct InputLine {
    inner: InnerWidget,
    length: usize,
    output_ready: bool,
    output: String,
    cursor_pos: u32,
    active: bool,
}

impl InputLine {
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
            active: true,
        }
    }

    pub fn is_active(&self) -> bool
    {
        self.active
    }

    pub fn set_active(&mut self)
    {
        self.active = true;
        self.inner.show_cursor();
        self.set_blanks(BLANK_CHAR);
    }

    pub fn set_inactive(&mut self)
    {
        self.active = false;
        self.inner.hide_cursor();
        self.set_blanks(INACTIVE_BLANK_CHAR);
    }

    fn set_blanks(&mut self, c: char)
    {
        let blank_count = self.length as isize - 1 - self.output.len() as isize;
        let first_blank = self.output.len() as u32;
        if blank_count > 0 {
            for x in first_blank..(first_blank + blank_count as u32) {
                self.inner.putc(0, x, c)
            }
        }
        self.inner.putc(0, self.length as u32 - 1, c);
    }
}

impl Widget for InputLine {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}

impl InteractiveWidget for InputLine {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Char('\n')) => {
                self.output_ready = true;
            },
            Event::Key(Key::Char(c)) => {
                if c.is_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                    let output_len = self.output.chars().count();

                    if output_len + 1 < self.length {
                        self.output.push(c);
                        self.inner.putc(0, self.cursor_pos, c);
                        self.inner.advance_cursor(1);
                        self.cursor_pos += 1;
                    } else {
                        self.output.push(c);
                        let output_len = output_len + 1;

                        self.inner.print(0, 0, self.output.as_str().slice_in_chars(output_len + 1 - self.length, output_len));
                    }
                }
            },
            Event::Key(Key::Backspace) => {
                if !self.output.is_empty() {
                    let output_len = self.output.chars().count();

                    if output_len + 1 <= self.length {
                        self.output.pop();
                        self.inner.putc(0, self.cursor_pos - 1, BLANK_CHAR);
                        self.inner.advance_cursor(-1);
                        self.cursor_pos -= 1;
                    } else {
                        self.output.pop();
                        let output_len = output_len - 1;

                        self.inner.print(0, 0, self.output.as_str().slice_in_chars(output_len + 1 - self.length, output_len));
                    }
                }
            },
            // TODO: arrow keys
            // TODO: Event::Key(Key::Delete) => {},
            _ => (),
        }
    }
}

impl OutputWidget<String> for InputLine {
    fn try_get_output(&self) -> Option<String>
    {
        if self.output_ready {
            return Some(self.output.clone());
        }
        None
    }

    fn get_output(&self) -> Result<String, PoisonError<String>>
    {
        let output = self.output.clone();

        if self.output_ready {
            return Ok(output);
        }
        Err(PoisonError::new(output))
    }
}

impl Aligned for InputLine {
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
}

impl Alignable for InputLine {
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

    fn adjust_pos(&mut self, y: i32, x: i32)
    {
        let mut inner = self.inner.borrow_mut();
        let new_y = inner.start_y as i32 + y;
        let new_x = inner.start_x as i32 + x;

        if new_y < 0 || new_x < 0 {
            panic!("position adjustment is out of bounds");
        }

        inner.start_y = new_y as u32;
        inner.start_x = new_x as u32;
    }

    fn change_pos(&mut self, y: u32, x: u32)
    {
        let mut inner = self.inner.borrow_mut();
        inner.start_y = y;
        inner.start_x = x;
    }
}
