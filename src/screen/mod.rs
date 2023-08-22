mod buffer;

use std::io::{Stdout, Write};
use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::MouseTerminal;

use crate::Area;
use crate::style::{Color, TextStyle};
use crate::util::offset;
use crate::widget::Widget;

pub use buffer::Buffer;

#[derive(Debug, Clone, Copy)]
struct InternalStyle {
    fg_color: Color,
    bg_color: Color,
    text_style: TextStyle,
}

impl Default for InternalStyle {
    fn default() -> Self
    {
        Self {
            fg_color: Color::Normal,
            bg_color: Color::Normal,
            text_style: TextStyle::NORMAL,
        }
    }
}

#[derive(Debug)]
pub struct RenderContext<'b> {
    buffer: Buffer<'b>,
}

impl<'b> RenderContext<'b> {
    pub fn area(&self) -> Area
    {
        let dim = self.buffer.area().dimensions();

        Area {
            x: 0,
            y: 0,
            width: dim.width,
            height: dim.height,
        }
    }

    pub fn render_widget<W: Widget>(&mut self, widget: &mut W, area: Area)
    {
        widget.render(&mut self.buffer, area);
    }
}

#[derive(Debug)]
struct Cursor {
    x: u16,
    y: u16,
    hidden: bool,
}

pub struct Screen {
    pub width: u16,
    pub height: u16,
    cursor: Cursor,
    char_buf: Vec<char>,
    style_buf: Vec<InternalStyle>,
    stdout: RawTerminal<MouseTerminal<Stdout>>,
}

impl Screen {
    pub fn init(rows: u16, cols: u16) -> Self
    {
        let (x, y) = termion::terminal_size()
            .expect("Failed to detect terminal size.");

        if cols > x || rows > y {
            panic!("terminal too small, needs to be at least: {cols}x{rows}");
        }

        let mut stdout = MouseTerminal::from(std::io::stdout())
            .into_raw_mode()
            .unwrap();

        console::hide_cursor(&mut stdout).unwrap();

        let buf_size = cols as usize * rows as usize;

        Self {
            width: cols,
            height: rows,
            char_buf: vec!['\0'; buf_size],
            style_buf: vec![InternalStyle::default(); buf_size],
            stdout,
            cursor: Cursor { y: 0, x: 0, hidden: true },
        }
    }

    pub fn draw<F>(&mut self, ui: F)
    where
        F: FnOnce(&mut RenderContext)
    {
        let mut ctx = RenderContext {
            buffer: Buffer::from_raw_parts(
                self.width,
                self.height,
                &mut self.char_buf,
                &mut self.style_buf,
                &mut self.cursor,
            )
        };

        ui(&mut ctx);
    }

    /// Writes the internal buffer to the terminal.
    pub fn refresh(&mut self)
    {
        for y in 0..self.height - 1 {
            self.write_line(y);
            console::write_str(&mut self.stdout, "\r\n").unwrap();
        }

        self.write_line(self.height - 1);
        console::write_char(&mut self.stdout, '\r').unwrap();
        console::move_cursor(&mut self.stdout, -(self.height as isize - 1), 0).unwrap();

        // TODO: implement cursor with a real cursor.
        if !self.cursor.hidden {
            // Move the cursor to the its position.
            console::move_cursor(
                &mut self.stdout,
                self.cursor.y as isize,
                self.cursor.x as isize
            ).unwrap();
            // char printing
            console::add_text_style(&mut self.stdout, TextStyle::INVERT).unwrap();
            console::write_char(
                &mut self.stdout,
                self.char_buf[offset!(
                    self.cursor.x as usize,
                    self.cursor.y as usize,
                    self.width as usize
                )]
            ).unwrap();
            console::subtract_text_style(&mut self.stdout, TextStyle::INVERT).unwrap();
            console::move_cursor(&mut self.stdout, 0, -1).unwrap();
            // Move the cursor back to the top left of the screen.
            console::move_cursor(
                &mut self.stdout,
                -(self.cursor.y as isize),
                -(self.cursor.x as isize)
            ).unwrap();
        }

        self.stdout.flush()
            .expect("failed to flush stdout");
    }

    fn write_line(&mut self, y: u16)
    {
        let width = self.width as usize;
        let line_offset = offset!(0, y as usize, width);
        let chars = &self.char_buf[line_offset..line_offset + width];
        let styles = &self.style_buf[line_offset..line_offset + width];

        // FIXME: optimise.

        let mut saved_fg = styles[0].fg_color;
        let mut saved_bg = styles[0].bg_color;
        let mut saved_ts = styles[0].text_style;
        // The first char of every line is always set with colors and style.
        console::set_fg_color(&mut self.stdout, saved_fg)
            .expect("failed to set fg color");
        console::set_bg_color(&mut self.stdout, saved_bg)
            .expect("failed to set bg color");
        console::set_text_style(&mut self.stdout, saved_ts)
            .expect("failed to set text style");
        console::write_char(&mut self.stdout, chars[0])
            .expect("failed to write a char to the screen");

        for x in 1..width {
            let cur_style = &styles[x];
            let cur_char = &chars[x];

            if saved_fg != cur_style.fg_color {
                console::set_fg_color(&mut self.stdout, cur_style.fg_color)
                    .expect("failed to set fg color");
                saved_fg = cur_style.fg_color;
            }
            if saved_bg != cur_style.bg_color {
                console::set_bg_color(&mut self.stdout, cur_style.bg_color)
                    .expect("failed to set bg color");
                saved_bg = cur_style.bg_color;
            }
            if saved_ts != cur_style.text_style {
                console::set_text_style(&mut self.stdout, cur_style.text_style)
                    .expect("failed to set text style");
                saved_ts = cur_style.text_style;
            }

            console::write_char(&mut self.stdout, *cur_char)
                .expect("failed to write a char to the screen");
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self)
    {
        console::set_fg_color(&mut self.stdout, Color::Normal).unwrap();
        console::set_bg_color(&mut self.stdout, Color::Normal).unwrap();
        console::set_text_style(&mut self.stdout, TextStyle::NORMAL).unwrap();
        for _row in 0..self.height {
            console::write_char(&mut self.stdout, '\n').unwrap();
        }
        console::show_cursor(&mut self.stdout).unwrap();
    }
}

#[allow(dead_code)]
mod console {
    use std::io::Write;
    use termion::color::{Bg, Fg};

    use crate::style::{Color, TextStyle};

    #[inline]
    pub fn write_char<W: Write>(writer: &mut W, c: char) -> Result<(), std::io::Error>
    {
        write!(writer, "{}", c)
    }

    #[inline]
    pub fn write_str<W: Write>(writer: &mut W, s: &str) -> Result<(), std::io::Error>
    {
        write!(writer, "{}", s)
    }

    #[inline]
    pub fn show_cursor<W: Write>(writer: &mut W) -> Result<(), std::io::Error>
    {
        write!(writer, "{}", termion::cursor::Show)
    }

    #[inline]
    pub fn hide_cursor<W: Write>(writer: &mut W) -> Result<(), std::io::Error>
    {
        write!(writer, "{}", termion::cursor::Hide)
    }

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
            Color::Normal       => write!(writer, "{}", Fg(termion::color::Reset))?,
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
            Color::Ansi(c)      => write!(writer, "{}", Fg(termion::color::AnsiValue(c)))?,
            Color::Rgb(r, g, b) => write!(writer, "{}", Fg(termion::color::Rgb(r, g, b)))?,
        }

        Ok(())
    }

    // FIXME: couldn't find a way to avoid duplication without `Box`ing the color code. Macros?

    #[inline]
    pub fn set_bg_color<W: Write>(writer: &mut W, color: Color) -> Result<(), std::io::Error>
    {
        match color {
            Color::Normal       => write!(writer, "{}", Bg(termion::color::Reset))?,
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
            Color::Ansi(c)      => write!(writer, "{}", Bg(termion::color::AnsiValue(c)))?,
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

    #[inline]
    pub fn add_text_style<W: Write>(writer: &mut W, ts: TextStyle) -> Result<(), std::io::Error>
    {
        if ts.contains(TextStyle::BOLD) {
            write!(writer, "{}", termion::style::Bold)?;
        }

        if ts.contains(TextStyle::BLINK) {
            write!(writer, "{}", termion::style::Blink)?;
        }

        if ts.contains(TextStyle::INVERT) {
            write!(writer, "{}", termion::style::Invert)?;
        }

        if ts.contains(TextStyle::ITALIC) {
            write!(writer, "{}", termion::style::Italic)?;
        }

        if ts.contains(TextStyle::UNDERLINE) {
            write!(writer, "{}", termion::style::Underline)?;
        }

        Ok(())
    }

    #[inline]
    pub fn subtract_text_style<W: Write>(writer: &mut W, ts: TextStyle) -> Result<(), std::io::Error>
    {
        if ts.contains(TextStyle::BOLD) {
            write!(writer, "{}", termion::style::NoBold)?;
        }

        if ts.contains(TextStyle::BLINK) {
            write!(writer, "{}", termion::style::NoBlink)?;
        }

        if ts.contains(TextStyle::INVERT) {
            write!(writer, "{}", termion::style::NoInvert)?;
        }

        if ts.contains(TextStyle::ITALIC) {
            write!(writer, "{}", termion::style::NoItalic)?;
        }

        if ts.contains(TextStyle::UNDERLINE) {
            write!(writer, "{}", termion::style::NoUnderline)?;
        }

        Ok(())
    }
}
