use termion::event::{Event, Key};

use super::{
    Widget,
    InteractiveWidget,
};
use crate::Pos;
use crate::layout::Area;
use crate::widget::Paint;
use crate::style::{StyledChar, Style, WithStyle};


/// Configuration options for theming [InputLine].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub blank_c: StyledChar,
    pub input_style: Style,
}

/// Primitive for rendering input fields.
#[derive(Debug, Clone)]
pub struct InputLine {
    pub theme: Theme,
    pub active: bool,
    content: String,
    cursor_pos: u16,
}

impl InputLine {
    /// Creates a new `InputLine` with the default capacity of `capacity`.
    pub fn with_capacity(capacity: usize) -> Self
    {
        Self {
            content: String::with_capacity(capacity),
            cursor_pos: 0,
            theme: Theme {
                blank_c: ' '.styled(),
                input_style: Style::default(),
            },
            active: false,
        }
    }

    /// Creates a new `InputLine`.
    pub fn new() -> Self
    {
        Self {
            content: String::new(),
            cursor_pos: 0,
            theme: Theme {
                blank_c: ' '.styled(),
                input_style: Style::default(),
            },
            active: false,
        }
    }

    /// Accesses the contents of the input.
    #[inline]
    pub fn content(&self) -> &str
    {
        &self.content
    }

    /// Adjusts the theme of the `InputLine`.
    #[inline]
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
            input_style,
        };

        self
    }
}

impl<P: Paint> Widget<P> for InputLine {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_collapsed() {
            return;
        }

        // Draw the input.

        let width = area.width as usize;
        // TODO: utf8 support (graphemes).
        let input_len = self.content.len();

        buf.hfill(area.top_left(), self.theme.blank_c, width);

        let capped_input_len = std::cmp::min(input_len, width - 1);
        let end = std::cmp::max(self.cursor_pos as usize, capped_input_len);
        let start = end.saturating_sub(width - 1);
        // TODO: utf8 support (graphemes).
        let visible_input = self.content[start..end]
            .with_style(|_| self.theme.input_style);

        buf.print(Pos::ZERO, visible_input, area);

        let cursor_moved = (self.cursor_pos as usize) < input_len;
        if cursor_moved && input_len >= width {
            buf.putc_abs(
                area.top_right().sub_x(1),
                // TODO: utf8 support (graphemes).
                self.content.chars().nth(self.cursor_pos as usize + 1)
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
            buf.show_cursor()
        }
    }
}

impl InteractiveWidget for InputLine {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Char(c)) => {
                // TODO: utf8 support.
                if c.is_ascii_alphanumeric()
                    || c.is_ascii_punctuation()
                    || c == ' '
                {
                    self.content.insert(self.cursor_pos as usize, c);
                    self.cursor_pos += 1;
                }
            },
            Event::Key(Key::Backspace) => {
                if self.cursor_pos > 0 {
                // TODO: utf8 support.
                    self.content.remove(self.cursor_pos as usize - 1);
                    self.cursor_pos -= 1;
                }
            },
            // TODO: arrow keys
            // TODO: Event::Key(Key::Delete) => {},
            _ => (),
        }
    }
}
