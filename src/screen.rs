use std::io::{stdout, Stdout, Write};
use crate::widget::Widget;
use crate::pos;

extern crate termion;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct Cursor {
    pub y: u32,
    pub x: u32,
    pub hidden: bool,
}

pub struct Screen {
    buffer: Vec<char>,
    height: usize,
    width: usize,
    pub cursor: Cursor,
    widgets: Vec<Widget>,
    pub overlay: Widget,
    stdout: RawTerminal<Stdout>,
}

impl Screen {
    pub fn init(rows: usize, cols: usize) -> Self
    {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        Self {
            buffer: vec![' '; cols * rows],
            height: rows,
            width: cols,
            cursor: Cursor {y: 0, x: 0, hidden: true},
            widgets: Vec::new(),
            overlay: Widget::new(0, 0, rows, cols),
            stdout,
        }
    }

    pub fn draw(&mut self)
    {
        self.widgets.sort();

        for c in &mut self.buffer {
            *c = ' ';
        }

        for i in 0..self.widgets.len() {
            if self.widgets[i].borrow().has_border {
                self.draw_widget_border(self.widgets[i].share());
            }
            self.draw_widget_content(self.widgets[i].share());
        }
        self.draw_widget_content(self.overlay.share());
    }

    pub fn refresh(&mut self)
    {
        for y in 0..self.height - 1 {
            for x in 0..self.width {
                write!(self.stdout, "{}", self.buffer[pos![self.width, y, x]]).unwrap();
            }
            write!(self.stdout, "\r\n").unwrap();
        }

        for x in 0..self.width {
            write!(self.stdout, "{}", self.buffer[pos![self.width, self.height - 1, x]]).unwrap();
        }
        write!(self.stdout, "\r{}", termion::cursor::Up(self.height as u16 - 1)).unwrap();

        if !self.cursor.hidden {
            // It has to be checked for zero values, as supplying 0 to the termion's cursor
            // movement functions will result in the cursor being moved by one position.
            if self.cursor.y != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Down(self.cursor.y as u16),
                ).unwrap();
            }
            if self.cursor.x != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Right(self.cursor.x as u16),
                ).unwrap();
            }

            write!(
                self.stdout,
                "{}{}{}{}",
                termion::style::Invert,
                self.buffer[pos![self.width, self.cursor.y as usize, self.cursor.x as usize]],
                termion::style::NoInvert,
                termion::cursor::Left(1),
            ).unwrap();

            if self.cursor.x != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Left(self.cursor.x as u16),
                ).unwrap();
            }
            if self.cursor.y != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Up(self.cursor.y as u16),
                ).unwrap();
            }
        }

        self.stdout.flush().unwrap();
    }

    pub fn add_widget(&mut self, start_y: u32, start_x: u32, height: usize, width: usize) -> Widget
    {
        let w = Widget::new(start_y, start_x, height, width);
        self.widgets.push(w.share());
        w
    }

    pub fn show_cursor(&mut self)
    {
        self.cursor.hidden = false;
    }

    pub fn hide_cursor(&mut self)
    {
        self.cursor.hidden = true;
    }

    pub fn move_cursor(&mut self, y: u32, x: u32)
    {
        if y as usize >= self.height || x as usize >= self.width {
            return;
        }

        self.cursor.y = y;
        self.cursor.x = x;
    }

    pub fn advance_cursor(&mut self, steps: i32)
    {
        if steps < 0 {
            if (-steps) as u32 > self.cursor.x {
                return;
            }
        } else if steps as u32 + self.cursor.x >= self.width as u32 {
            return;
        }

        self.cursor.x = (self.cursor.x as i32 + steps) as u32;
    }

    fn draw_widget_border(&mut self, w: Widget)
    {
        let w = w.borrow();

        let width = w.width as usize;
        let height = w.height as usize;
        let start_y = w.start_y as usize;
        let start_x = w.start_x as usize;
        let border_chars = w.border_style;

        let sw = self.width;

        if border_chars.0 != '\0' {
            for i in 0..width {
                self.buffer[pos![sw, start_y, start_x + i]] = border_chars.0;
                self.buffer[pos![sw, start_y + height - 1, start_x + i]] = border_chars.0;
            }
        }
        if border_chars.1 != '\0' {
            for i in 0..height {
                self.buffer[pos![sw, start_y + i, start_x]] = border_chars.1;
                self.buffer[pos![sw, start_y + i, start_x + width - 1]] = border_chars.1;
            }
        }
        if border_chars.2 != '\0' {
            self.buffer[pos![sw, start_y, start_x]] = border_chars.2;
        }
        if border_chars.3 != '\0' {
            self.buffer[pos![sw, start_y, start_x + width - 1]] = border_chars.3;
        }
        if border_chars.4 != '\0' {
            self.buffer[pos![sw, start_y + height - 1, start_x + width - 1]] = border_chars.4;
        }
        if border_chars.5 != '\0' {
            self.buffer[pos![sw, start_y + height - 1, start_x]] = border_chars.5;
        }
    }

    fn draw_widget_content(&mut self, w: Widget)
    {
        let w = w.borrow();

        let mut width = w.width;
        let mut height = w.height;
        let mut start_x = w.start_x as usize;
        let mut start_y = w.start_y as usize;

        if w.has_border {
            if width <= 2 || height <= 2 {
                return;
            }

            width -= 2;
            height -= 2;
            start_x += 1;
            start_y += 1;
        }

        let ww = w.width;
        let sw = self.width;

        for y in 0..height {
            for x in 0..width {
                let c = w.buffer[pos![ww, y, x]];

                if c == '\0' {
                    continue;
                }

                self.buffer[pos![sw, start_y + y, start_x + x]] = c;
            }
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self)
    {
        for _ in 0..self.height {
            write!(self.stdout, "\n").unwrap();
        }
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }
}
