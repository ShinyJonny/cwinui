use termion::event::{Event, Key};

use super::{
    Draw,
    InteractiveWidget,
};
use crate::Pos;
use crate::layout::{Area, Proportional, Proportions};
use crate::widget::Paint;
use crate::style::{StyledChar, Style, WithStyle};


/// Configuration options for theming [`InputLine`].
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub blank_c: StyledChar,
    pub input_style: Style,
}

impl Theme {
    /// Const version of `Default::default`.
    pub const fn default() -> Self
    {
        Self {
            blank_c: StyledChar { content: ' ', style: Style::default() },
            input_style: Style::default(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self
    {
        Self::default()
    }
}

/// Primitive for drawing input fields.
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
            theme: Theme::default(),
            active: false,
        }
    }

    /// Creates a new `InputLine`.
    pub const fn new() -> Self
    {
        Self {
            content: String::new(),
            cursor_pos: 0,
            theme: Theme::default(),
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
    pub const fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

        self
    }
}

impl<P: Paint> Draw<P> for InputLine {
    fn draw(&self, buf: &mut P, area: Area)
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

impl Proportional for InputLine {
    fn proportions(&self) -> Proportions
    {
        use crate::layout::Range;

        Proportions {
            height: Range::flexible(),
            width: Range::fixed(1),
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
