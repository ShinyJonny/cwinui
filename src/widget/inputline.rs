use termion::event::{Event, Key};

use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    PoisonError,
};
use crate::Pos;
use crate::misc::SliceByChars;
use crate::layout::Area;
use crate::screen::Buffer;
use crate::style::{StyledChar, Style, WithStyle};

const INPUT_CAPACITY: usize = 2048;

struct Theme {
    blank_c: StyledChar,
    input_style: Style,
}

pub struct InputLine {
    last_width: u16,
    output_ready: bool,
    input: String,
    cursor_pos: u16,
    theme: Theme,
}

impl InputLine {
    pub fn new(pos: Pos, length: u16) -> Self
    {
        Self {
            last_width: 0,
            output_ready: false,
            input: String::with_capacity(INPUT_CAPACITY),
            cursor_pos: 0,
            theme: Theme {
                blank_c: ' '.styled(),
                input_style: Style::default(),
            },
        }
    }

    pub fn theme<C>(
        mut self,
        blank_c: C,
        input_style: Style,
    ) -> Self
    where
        C: Into<StyledChar>
    {
        self.set_theme(blank_c, input_style);

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
    }
}

impl Widget for InputLine {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        self.last_width = area.width;

        // FIXME: this needs a review, especially the cursor position and the
        // text after the cursor (under).

        // Draw the input.

        let input_len = self.input.chars().count();
        let visible_input = if input_len + 1 < area.width as usize {
            self.input.as_str()
        } else {
            self.input.slice_by_chars(
                input_len + 1 - area.width as usize..input_len
            )
        };
        buf.print(area.x, area.y,
            visible_input.with_style(|_| self.theme.input_style));

        // Draw the blanks.

        if input_len + 1 < area.width as usize {
            let blank_count = area.width as usize - 1 - input_len;
            let first_blank_x = area.x + input_len as u16;
            buf.hfill(first_blank_x, area.y, self.theme.blank_c,
                 blank_count as usize);
        }
        let Pos { x, y } = area.top_right().sub_x(1);
        buf.putc(x, y, self.theme.blank_c);

        // FIXME: this is wrong, as the cursor can be bigger.
        buf.move_cursor(area.top_left().add_x(self.cursor_pos));
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
                    self.input.push(c);
                    self.cursor_pos += 1;
                }
            },
            Event::Key(Key::Backspace) => {
                if !self.input.is_empty() {
                    self.input.pop();
                    self.cursor_pos -= 1;
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
