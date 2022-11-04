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
use crate::style::{StyledChar, Style, WithStyle};

const INPUT_CAPACITY: usize = 2048;

struct Theme {
    blank_c: StyledChar,
    input_style: Style,
}

pub struct InputLine {
    inner: InnerWidget,
    length: usize,
    output_ready: bool,
    input: String,
    cursor_pos: u32,
    theme: Theme,
}

impl InputLine {
    pub fn new(y: u32, x: u32, length: usize) -> Self
    {
        let theme = Theme {
            blank_c: ' '.styled(),
            input_style: Style::default(),
        } ;
        let inner = InnerWidget::new(y, x, 1, length);
        inner.show_cursor();

        let mut il = Self {
            inner,
            length,
            output_ready: false,
            input: String::with_capacity(INPUT_CAPACITY),
            cursor_pos: 0,
            theme,
        };
        il.redraw();

        il
    }

    pub fn theme<C>(
        mut self,
        blank_c: C,
        input_style: Style,
    ) -> Self
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            blank_c: blank_c.styled(),
            input_style,
        };
        self.redraw();

        self
    }

    pub fn set_theme<C>(
        &mut self,
        blank_c: C,
        input_style: Style,
    )
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            blank_c: blank_c.styled(),
            input_style,
        };
        self.redraw();
    }

    pub fn redraw(&mut self)
    {
        // Draw the input.

        let input_len = self.input.chars().count();
        let visible_input = if input_len + 1 < self.length {
            self.input.as_str()
        } else {
            self.input.slice_in_chars(input_len + 1 - self.length, input_len)
        };
        self.inner.print(0, 0, visible_input.with_style(|_| self.theme.input_style));

        // Draw the blanks.

        let blank_count = self.length as isize - 1 - input_len as isize;
        let first_blank_x = input_len as u32;
        if blank_count > 0 {
            self.inner.hfill(0, first_blank_x, self.theme.blank_c, blank_count as usize);
        }
        self.inner.putc(0, self.length as u32 - 1, self.theme.blank_c);

        self.inner.move_cursor(0, self.cursor_pos);
    }

    pub fn resize(&mut self, len: usize)
    {
        if len < 1 {
            panic!("input line cannot be resized below 1");
        }

        self.inner.resize(1, len);
        self.length = len;

        self.cursor_pos = if self.input.len() + 1 > self.length {
            self.length as u32
        } else {
            self.input.len() as u32
        };

        self.redraw();
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
                    let input_len = self.input.chars().count();

                    if input_len + 1 < self.length {
                        self.cursor_pos += 1;
                    }
                    self.input.push(c);

                    self.redraw();
                }
            },
            Event::Key(Key::Backspace) => {
                if !self.input.is_empty() {
                    let input_len = self.input.chars().count();

                    if input_len + 1 <= self.length {
                        self.cursor_pos -= 1;
                    }
                    self.input.pop();

                    self.redraw();
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
            return Some(self.input.clone());
        }
        None
    }

    fn get_output(&self) -> Result<String, PoisonError<String>>
    {
        let output = self.input.clone();

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
