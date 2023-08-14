use termion::event::{Event, Key};

use super::{
    InnerWidget,
    Widget,
    InteractiveWidget,
    OutputWidget,
    PoisonError,
};
use crate::Pos;
use crate::misc::SliceInChars;
use crate::layout::Area;
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
    pub fn new(pos: Pos, length: usize) -> Self
    {
        let theme = Theme {
            blank_c: ' '.styled(),
            input_style: Style::default(),
        } ;
        let inner = InnerWidget::new(Area {
            x: pos.x,
            y: pos.y,
            width: length,
            height: 1
        });
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
            self.inner.hfill(first_blank_x, 0, self.theme.blank_c, blank_count as usize);
        }
        self.inner.putc(self.length as u32 - 1, 0, self.theme.blank_c);

        self.inner.move_cursor(self.cursor_pos, 0);
    }

    pub fn resize(&mut self, len: usize)
    {
        if len < 1 {
            panic!("input line cannot be resized below 1");
        }

        self.inner.resize(len, 1);
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
