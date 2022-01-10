use crate::widget::{InnerWidget, Widget, InteractiveWidget, OutputWidget};
use crate::misc::PoisonError;

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
                        // TODO: cusor advancement
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
                        // TODO: cursor advancement
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
