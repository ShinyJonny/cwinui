use std::io::{Stdout, Write};
use crate::widget::Widget;
use crate::widget::InnerWidget;
use crate::pos;

extern crate termion;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct Cursor {
    y: u32,
    x: u32,
    hidden: bool,
}

pub struct Screen {
    buffer: Vec<char>,
    height: usize,
    width: usize,
    cursor: Cursor,
    widgets: Vec<InnerWidget>,
    stdout: RawTerminal<Stdout>,
}

impl Screen {
    pub fn init(rows: usize, cols: usize) -> Self
    {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        Self {
            buffer: vec![' '; cols * rows],
            height: rows,
            width: cols,
            cursor: Cursor {y: 0, x: 0, hidden: true},
            widgets: Vec::new(),
            stdout,
        }
    }

    pub fn draw(&mut self)
    {
        self.widgets.sort_by(|a, b| {
            a.borrow().z_index.cmp(&b.borrow().z_index)
        });

        for c in &mut self.buffer {
            *c = ' ';
        }

        for i in 0..self.widgets.len() {
            let hidden = self.widgets[i].borrow().hidden;
            if !hidden {
                self.draw_widget(self.widgets[i].share());
            }
        }
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

    pub fn add_widget<T>(&mut self, w: &T)
        where T: Widget
    {
        self.widgets.push(w.share_inner());
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

    fn draw_widget(&mut self, w: InnerWidget)
    {
        let w = w.borrow();

        let start_x = w.start_x as usize;
        let start_y = w.start_y as usize;

        let mut y_iterations = w.height;
        let mut x_iterations = w.width;
        if start_y + w.height > self.height {
            y_iterations = self.height - start_y;
        }
        if start_x + w.width > self.width {
            x_iterations = self.width - start_x;
        }

        let ww = w.width;
        let sw = self.width;

        for y in 0..y_iterations {
            for x in 0..x_iterations {
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
