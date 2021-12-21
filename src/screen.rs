use std::io::{stdin, stdout, Write, Stdin, Stdout};
use crate::widget::Widget;

extern crate termion;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

macro_rules! pos {
    ( $width:expr, $y:expr, $x:expr ) => {
        $y * $width + $x
    }
}

pub struct Screen {
    buffer: Vec<char>,
    height: usize,
    width: usize,
    widgets: Vec<Widget>,
    overlay: Widget,
    stdout: RawTerminal<Stdout>,
    stdin: Stdin,
}

impl Screen {
    pub fn new(rows: usize, cols: usize) -> Self
    {
        let stdout = stdout().into_raw_mode().unwrap();
        stdout.suspend_raw_mode().unwrap();

        Self {
            buffer: vec![' '; cols * rows],
            height: rows,
            width: cols,
            widgets: Vec::new(),
            overlay: Widget::new(0, 0, rows, cols),
            stdin: stdin(),
            stdout,
        }
    }

    pub fn init(&mut self)
    {
        self.stdout.activate_raw_mode().unwrap();
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();
    }

    pub fn draw(&mut self)
    {
        self.widgets.sort();

        for c in &mut self.buffer {
            *c = ' ';
        }

        for i in 0..self.widgets.len() {
            if self.widgets[i].borrow().has_border {
                self.draw_border(self.widgets[i].share());
            }
            self.draw_widget(self.widgets[i].share());
        }
        self.draw_widget(self.overlay.share());
    }

    // TODO: cursor movement (restoration) and figure out edge cases.
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

        self.stdout.flush().unwrap();
    }

    pub fn add_widget(&mut self, start_y: u32, start_x: u32, height: usize, width: usize) -> Widget
    {
        let w = Widget::new(start_y, start_x, height, width);
        self.widgets.push(w.share());
        w
    }

    fn draw_border(&mut self, w: Widget)
    {
        let w = w.borrow();

        let width = w.width as usize;
        let height = w.height as usize;
        let start_y = w.start_y as usize;
        let start_x = w.start_x as usize;
        let border_chars = w.border_style;

        let sw = self.width;

        if border_chars.0 != '\0' {
            for i in 0..(width - 1) {
                self.buffer[pos![sw, start_y, start_x + i]] = border_chars.0;
                self.buffer[pos![sw, start_y + height - 1, start_x + i]] = border_chars.0;
            }
        }
        if border_chars.1 != '\0' {
            for i in 0..(height - 1) {
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

    // FIXME: Passing overlay as index -1 is just lazy.
    fn draw_widget(&mut self, w: Widget)
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
