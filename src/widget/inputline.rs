use termion::event::{Event, Key};

use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    PoisonError,
};
use crate::Pos;
use crate::layout::Area;
use crate::screen::Buffer;
use crate::style::{StyledChar, Style, WithStyle};

const INPUT_CAPACITY: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub blank_c: StyledChar,
    pub input_style: Style,
}

#[derive(Debug, Clone)]
pub struct InputLine {
    pub theme: Theme,
    last_width: u16,
    output_ready: bool,
    input: String,
    cursor_pos: u16,
    active: bool,
}

impl InputLine {
    pub fn new() -> Self
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
            active: true,
        }
    }

    pub fn set_active(&mut self)
    {
        self.active = true;
    }

    pub fn set_inactive(&mut self)
    {
        self.active = false;
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
            blank_c: blank_c.into(),
            input_style: input_style.into(),
        };

        self
    }
}

impl Widget for InputLine {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        self.last_width = area.width;

        // Draw the input.

        let width = area.width as usize;
        // TODO: utf8 support (graphemes).
        let input_len = self.input.len();

        buf.hfill(area.x, area.y, self.theme.blank_c, width);

        let capped_input_len = std::cmp::min(input_len, width - 1);
        let end = std::cmp::max(self.cursor_pos as usize, capped_input_len);
        let start = end.saturating_sub(width - 1);
        // TODO: utf8 support (graphemes).
        let visible_input = &self.input[start..end];

        buf.print(area.x, area.y,
            visible_input.with_style(|_| self.theme.input_style));

        let cursor_moved = (self.cursor_pos as usize) < input_len;
        if cursor_moved && input_len >= width {
            let Pos { x, y } = area.top_right().sub_x(1);
            buf.putc(
                x,
                y,
                // TODO: utf8 support (graphemes).
                self.input.chars().nth(self.cursor_pos as usize + 1)
                    .unwrap()
                    .with_style(|_| self.theme.input_style),
            );
        }

        if self.active {
            buf.move_cursor(Pos {
                x: std::cmp::min(
                    area.x + self.cursor_pos,
                    area.x + area.width - 1,
                ),
                y: area.y
            });
        }
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
                // TODO: utf8 support.
                if c.is_ascii_alphanumeric()
                    || c.is_ascii_punctuation()
                    || c == ' '
                {
                    self.input.insert(self.cursor_pos as usize, c);
                    self.cursor_pos += 1;
                }
            },
            Event::Key(Key::Backspace) => {
                if self.cursor_pos > 0 {
                // TODO: utf8 support.
                    self.input.remove(self.cursor_pos as usize - 1);
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
