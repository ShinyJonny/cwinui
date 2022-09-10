use std::io::{Stdout, Write};
use std::rc::Rc;
use std::ops::Deref;
use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::MouseTerminal;

use crate::style::{Color, TextStyle};
use crate::widget::Widget;
use crate::widget::InnerWidget;
use crate::pos;

#[derive(Clone, Copy)]
struct InternalStyle {
    fg_color: Color,
    bg_color: Color,
    text_style: TextStyle,
}

impl Default for InternalStyle {
    fn default() -> Self {
        InternalStyle {
            fg_color: Color::Normal,
            bg_color: Color::Normal,
            text_style: TextStyle::NORMAL,
        }
    }
}

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
    style_buffer: Vec<InternalStyle>,
    stdout: RawTerminal<MouseTerminal<Stdout>>,
    widgets: Vec<InnerWidget>,
}

impl Screen {
    pub fn init(rows: usize, cols: usize) -> Self
    {
        let (x, y) = termion::terminal_size()
            .expect("Failed to detect terminal size.");
        let y = y as usize;
        let x = x as usize;

        if rows > y || cols > x {
            panic!("terminal too small, needs to be at least: {cols}x{rows}");
        }

        let mut stdout = MouseTerminal::from(std::io::stdout()).into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        Self {
            height: rows,
            width: cols,
            dtime: 0f64,
            buffer: vec![' '; cols * rows],
            style_buffer: vec![InternalStyle::default(); cols * rows],
            stdout,
            widgets: Vec::new(),
            cursor: Cursor { y: 0, x: 0, hidden: true },
        }
    }

    /// Refreshes the screen.
    pub fn refresh(&mut self)
    {
        self.draw();
        self.render();
    }

    /// Adds a widget to the screen.
    pub fn add_widget<T: Widget>(&mut self, w: &T)
    {
        self.widgets.push(w.share_inner());
    }

    /// Removes a widget from the screen.
    pub fn rm_widget<T: Widget>(&mut self, w: &T)
    {
        let w = w.share_inner();

        if let Some(i) = self.widgets.iter().position(|wid| {
            std::ptr::eq(
                Rc::deref(InnerWidget::deref(&w)),
                Rc::deref(InnerWidget::deref(wid))
            )
        }) {
            self.widgets.remove(i);
        }
    }

    /// Writes the internal buffer to the terminal.
    fn render(&mut self)
    {
        for y in 0..self.height - 1 {
            self.render_line(y);
            write!(self.stdout, "\r\n").unwrap();
        }

        self.render_line(self.height - 1);
        write!(self.stdout, "\r{}", termion::cursor::Up(self.height as u16 - 1)).unwrap();

        // TODO: implement cursor with a real cursor.
        if !self.cursor.hidden {
            // Move the cursor to the its position.
            render::move_cursor(
                &mut self.stdout,
                self.cursor.y as isize,
                self.cursor.x as isize
            ).unwrap();
            // char printing
            write!(
                self.stdout,
                "{}{}{}{}",
                termion::style::Invert,
                self.buffer[pos![self.width, self.cursor.y as usize, self.cursor.x as usize]],
                termion::style::NoInvert,
                termion::cursor::Left(1),
            ).unwrap();
            // Move the cursor back to the top left of the screen.
            render::move_cursor(
                &mut self.stdout,
                -(self.cursor.y as isize),
                -(self.cursor.x as isize)
            ).unwrap();
        }

        self.stdout.flush()
            .expect("failed to flush stdout");
    }

    fn render_line(&mut self, y: usize)
    {
        let line_offset = pos![self.width, y, 0];
        let chars = &self.buffer[line_offset..line_offset + self.width];
        let styles = &self.style_buffer[line_offset..line_offset + self.width];

        // FIXME: optimise.

        let mut saved_fg = styles[0].fg_color;
        let mut saved_bg = styles[0].bg_color;
        let mut saved_ts = styles[0].text_style;
        // The first char of every line is always set with colors and style.
        render::set_fg_color(&mut self.stdout, saved_fg)
            .expect("failed to set fg color");
        render::set_bg_color(&mut self.stdout, saved_bg)
            .expect("failed to set bg color");
        render::set_text_style(&mut self.stdout, saved_ts)
            .expect("failed to set text style");
        write!(self.stdout, "{}", chars[0])
            .expect("failed to write a char to the screen");

        for x in 1..self.width {
            let cur_style = &styles[x];
            let cur_char = &chars[x];

            if saved_fg != cur_style.fg_color {
                render::set_fg_color(&mut self.stdout, cur_style.fg_color)
                    .expect("failed to set fg color");
                saved_fg = cur_style.fg_color;
            }
            if saved_bg != cur_style.bg_color {
                render::set_bg_color(&mut self.stdout, cur_style.bg_color)
                    .expect("failed to set bg color");
                saved_bg = cur_style.bg_color;
            }
            if saved_ts != cur_style.text_style {
                render::set_text_style(&mut self.stdout, cur_style.text_style)
                    .expect("failed to set text style");
                saved_ts = cur_style.text_style;
            }

            write!(self.stdout, "{}", cur_char)
                .expect("failed to write a char to the screen");
        }
    }

    /// Constructs the internal buffer from all the widgets.
    fn draw(&mut self)
    {
        self.buffer.fill(' ');
        self.style_buffer.fill(InternalStyle::default());

        self.cursor.hidden = true;

        self.widgets.sort_by(|a, b| {
            a.borrow().z_index.cmp(&b.borrow().z_index)
        });

        for i in 0..self.widgets.len() {
            self.draw_widget(self.widgets[i].share());
        }
    }

    fn draw_widget(&mut self, w: InnerWidget)
    {
        if w.borrow().hidden {
            return;
        }

        self.draw_widget_buffers(w.share());

        // NOTE: Doesn't support multiple cursors. The cursor position of the top widget with a
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

    fn draw_widget_buffers(&mut self, w: InnerWidget)
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
                let w_pos = pos![ww, y, x];
                let s_pos = pos![sw, start_y + y, start_x + x];

                let c = w.buffer[w_pos];

                if c != '\0' {
                    self.buffer[s_pos] = c;
                }

                let fg_color = w.style_buffer[w_pos].fg_color;
                let bg_color = w.style_buffer[w_pos].bg_color;
                let text_style = w.style_buffer[w_pos].text_style;

                if let Some(color) = fg_color {
                    self.style_buffer[s_pos].fg_color = color;
                }
                if let Some(color) = bg_color {
                    self.style_buffer[s_pos].bg_color = color;
                }
                if let Some(ts) = text_style {
                    self.style_buffer[s_pos].text_style = ts;
                }
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

mod render {
    use std::io::Write;
    use termion::color::{Bg, Fg};

    use crate::style::{Color, TextStyle};

    #[inline]
    pub fn move_cursor<W: Write>(writer: &mut W, y: isize, x: isize) -> Result<(), std::io::Error>
    {
        // NOTE: it has to be checked for zero values, as supplying 0 to the termion's cursor
        // movement functions will result in the cursor being moved by one position.

        // y movement
        if y != 0 {
            if y < 0 {
                write!(writer, "{}", termion::cursor::Up((-y) as u16))?;
            } else {
                write!(writer, "{}", termion::cursor::Down(y as u16))?;
            }
        }
        // x movement
        if x != 0 {
            if x < 0 {
                write!(writer, "{}", termion::cursor::Left((-x) as u16))?;
            } else {
                write!(writer, "{}", termion::cursor::Right(x as u16))?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn set_fg_color<W: Write>(writer: &mut W, color: Color) -> Result<(), std::io::Error>
    {
        match color {
            Color::Normal => write!(writer, "{}", Fg(termion::color::Reset))?,
            Color::Black        => write!(writer, "{}", Fg(termion::color::Black))?,
            Color::Red          => write!(writer, "{}", Fg(termion::color::Red))?,
            Color::Green        => write!(writer, "{}", Fg(termion::color::Green))?,
            Color::Yellow       => write!(writer, "{}", Fg(termion::color::Yellow))?,
            Color::Blue         => write!(writer, "{}", Fg(termion::color::Blue))?,
            Color::Magenta      => write!(writer, "{}", Fg(termion::color::Magenta))?,
            Color::Cyan         => write!(writer, "{}", Fg(termion::color::Cyan))?,
            Color::White        => write!(writer, "{}", Fg(termion::color::White))?,
            Color::LightBlack   => write!(writer, "{}", Fg(termion::color::LightBlack))?,
            Color::LightRed     => write!(writer, "{}", Fg(termion::color::LightRed))?,
            Color::LightGreen   => write!(writer, "{}", Fg(termion::color::LightGreen))?,
            Color::LightYellow  => write!(writer, "{}", Fg(termion::color::LightYellow))?,
            Color::LightBlue    => write!(writer, "{}", Fg(termion::color::LightBlue))?,
            Color::LightMagenta => write!(writer, "{}", Fg(termion::color::LightMagenta))?,
            Color::LightCyan    => write!(writer, "{}", Fg(termion::color::LightCyan))?,
            Color::LightWhite   => write!(writer, "{}", Fg(termion::color::LightCyan))?,
            Color::Ansi(c)     => write!(writer, "{}", Fg(termion::color::AnsiValue(c)))?,
            Color::Rgb(r, g, b) => write!(writer, "{}", Fg(termion::color::Rgb(r, g, b)))?,
        }

        Ok(())
    }

    // FIXME: couldn't find a way to avoid duplication without `Box`ing the color code. Macros?

    #[inline]
    pub fn set_bg_color<W: Write>(writer: &mut W, color: Color) -> Result<(), std::io::Error>
    {
        match color {
            Color::Normal => write!(writer, "{}", Bg(termion::color::Reset))?,
            Color::Black        => write!(writer, "{}", Bg(termion::color::Black))?,
            Color::Red          => write!(writer, "{}", Bg(termion::color::Red))?,
            Color::Green        => write!(writer, "{}", Bg(termion::color::Green))?,
            Color::Yellow       => write!(writer, "{}", Bg(termion::color::Yellow))?,
            Color::Blue         => write!(writer, "{}", Bg(termion::color::Blue))?,
            Color::Magenta      => write!(writer, "{}", Bg(termion::color::Magenta))?,
            Color::Cyan         => write!(writer, "{}", Bg(termion::color::Cyan))?,
            Color::White        => write!(writer, "{}", Bg(termion::color::White))?,
            Color::LightBlack   => write!(writer, "{}", Bg(termion::color::LightBlack))?,
            Color::LightRed     => write!(writer, "{}", Bg(termion::color::LightRed))?,
            Color::LightGreen   => write!(writer, "{}", Bg(termion::color::LightGreen))?,
            Color::LightYellow  => write!(writer, "{}", Bg(termion::color::LightYellow))?,
            Color::LightBlue    => write!(writer, "{}", Bg(termion::color::LightBlue))?,
            Color::LightMagenta => write!(writer, "{}", Bg(termion::color::LightMagenta))?,
            Color::LightCyan    => write!(writer, "{}", Bg(termion::color::LightCyan))?,
            Color::LightWhite   => write!(writer, "{}", Bg(termion::color::LightCyan))?,
            Color::Ansi(c)     => write!(writer, "{}", Bg(termion::color::AnsiValue(c)))?,
            Color::Rgb(r, g, b) => write!(writer, "{}", Bg(termion::color::Rgb(r, g, b)))?,
        }

        Ok(())
    }

    #[inline]
    pub fn set_text_style<W: Write>(writer: &mut W, ts: TextStyle) -> Result<(), std::io::Error>
    {
        if ts.contains(TextStyle::BOLD) {
            write!(writer, "{}", termion::style::Bold)?;
        } else {
            write!(writer, "{}", termion::style::NoBold)?;
        }

        if ts.contains(TextStyle::BLINK) {
            write!(writer, "{}", termion::style::Blink)?;
        } else {
            write!(writer, "{}", termion::style::NoBlink)?;
        }

        if ts.contains(TextStyle::INVERT) {
            write!(writer, "{}", termion::style::Invert)?;
        } else {
            write!(writer, "{}", termion::style::NoInvert)?;
        }

        if ts.contains(TextStyle::ITALIC) {
            write!(writer, "{}", termion::style::Italic)?;
        } else {
            write!(writer, "{}", termion::style::NoItalic)?;
        }

        if ts.contains(TextStyle::UNDERLINE) {
            write!(writer, "{}", termion::style::Underline)?;
        } else {
            write!(writer, "{}", termion::style::NoUnderline)?;
        }

        Ok(())
    }
}
