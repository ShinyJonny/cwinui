use std::io::{Stdout, Write};
use std::time::Instant;
use std::rc::Rc;
use std::ops::Deref;
use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::MouseTerminal;

use crate::widget::Widget;
use crate::widget::InnerWidget;
use crate::pos;

struct Cursor {
    y: u32,
    x: u32,
    hidden: bool,
}

pub struct Screen {
    pub height: usize,
    pub width: usize,
    pub dtime: f64,
    cursor: Cursor,
    buffer: Vec<char>,
    stdout: RawTerminal<MouseTerminal<Stdout>>,
    widgets: Vec<InnerWidget>,
    last_refresh: Instant,
}

impl Screen {
    pub fn init(rows: usize, cols: usize) -> Self
    {
        let (x, y) = termion::terminal_size().unwrap();
        if rows > y as usize || cols > x as usize {
            panic!("terminal too small, needs to be at least: {}x{}", cols, rows);
        }

        let mut stdout = MouseTerminal::from(std::io::stdout()).into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        Self {
            height: rows,
            width: cols,
            dtime: 0f64,
            buffer: vec![' '; cols * rows],
            stdout,
            widgets: Vec::new(),
            cursor: Cursor { y: 0, x: 0, hidden: true },
            last_refresh: Instant::now(),
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

            // y movement
            if self.cursor.y != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Down(self.cursor.y as u16),
                ).unwrap();
            }
            // x movement
            if self.cursor.x != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Right(self.cursor.x as u16),
                ).unwrap();
            }
            // char printing
            write!(
                self.stdout,
                "{}{}{}{}",
                termion::style::Invert,
                self.buffer[pos![self.width, self.cursor.y as usize, self.cursor.x as usize]],
                termion::style::NoInvert,
                termion::cursor::Left(1),
            ).unwrap();
            // move x back
            if self.cursor.x != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Left(self.cursor.x as u16),
                ).unwrap();
            }
            // move y back
            if self.cursor.y != 0 {
                write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Up(self.cursor.y as u16),
                ).unwrap();
            }
        }

        self.stdout.flush().unwrap();

        let new_time = Instant::now();
        let diff = new_time.duration_since(self.last_refresh);
        self.last_refresh = new_time;
        self.dtime = diff.as_secs_f64();
    }

    pub fn draw(&mut self)
    {
        for c in &mut self.buffer {
            *c = ' ';
        }

        self.cursor.hidden = true;

        self.widgets.sort_by(|a, b| {
            a.borrow().z_index.cmp(&b.borrow().z_index)
        });

        for i in 0..self.widgets.len() {
            self.draw_widget(self.widgets[i].share());
        }
    }

    pub fn add_widget<T>(&mut self, w: &T)
        where T: Widget
    {
        self.widgets.push(w.share_inner());
    }

    pub fn rm_widget<T: Widget>(&mut self, w: &T)
    {
        let w = w.share_inner();

        // TODO: simplify with `.iter().position(...)`

        for i in 0..self.widgets.len() {
            let same_widgets = std::ptr::eq(
                Rc::deref(InnerWidget::deref(&w)),
                Rc::deref(InnerWidget::deref(&self.widgets[i]))
            );
            if same_widgets {
                self.widgets.remove(i);
                break;
            }
        }
    }

    fn draw_widget(&mut self, w: InnerWidget)
    {
        if w.borrow().hidden {
            return;
        }

        self.draw_widget_buffer(w.share());

        // TODO: Doesn't support multiple cursors. The cursor position of the top widget with a
        // shown cursor is used.
        let inner = w.borrow();
        if !inner.cursor.hidden {
            let start_y = inner.start_y;
            let start_x = inner.start_x;
            let cursor_y = inner.cursor.y;
            let cursor_x = inner.cursor.x;

            self.move_cursor(start_y + cursor_y, start_x + cursor_x);
            self.cursor.hidden = false;
        }
        drop(inner);

        w.borrow_mut().subwidgets.sort_by(|a, b| {
            a.borrow().z_index.cmp(&b.borrow().z_index)
        });

        for subw in &w.borrow().subwidgets {
            self.draw_widget(subw.share())
        }
    }

    fn draw_widget_buffer(&mut self, w: InnerWidget)
    {
        // FIXME: check for non-printable and variable-length characters (including whitespace).

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

    fn move_cursor(&mut self, y: u32, x: u32)
    {
        if y as usize >= self.height || x as usize >= self.width {
            return;
        }

        self.cursor.y = y;
        self.cursor.x = x;
    }

    // NOTE: might be deprecated
    fn advance_cursor(&mut self, steps: i32)
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
}

impl Drop for Screen {
    fn drop(&mut self)
    {
        for _row in 0..self.height {
            write!(self.stdout, "\n").unwrap();
        }
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }
}
