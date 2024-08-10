use super::Backend;


pub mod alloc {
    use std::io::{Stdout, Write};
    use termion::raw::{RawTerminal, IntoRawMode};
    use termion::input::MouseTerminal;

    use crate::buffer::{Buffer, Cursor};
    use crate::style::{Style, Color, TextStyle};
    use crate::util::offset;
    use crate::render::Render;

    use super::{Backend, console};


    /// Termion-based fixed-size backend.
    pub struct TermionFixed<const WIDTH: u16, const HEIGHT: u16> {
        // FIXME: when `generic_const_exprs` get stabilised, change this to
        // regular arrays and move this out of `alloc`. Can termion even
        // function in a no-alloc environment?
        chars: Box<[char]>,
        styles: Box<[Style]>,
        cursor: Cursor,
        stdout: RawTerminal<MouseTerminal<Stdout>>,
    }

    impl<const W: u16, const H: u16> std::fmt::Debug for TermionFixed<W, H> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        {
            f.write_fmt(format_args!("TermionFixed<{W}, {H}>"))
        }
    }

    impl<const W: u16, const H: u16> TermionFixed<W, H> {
        /// Initialises and creates the backend.
        ///
        /// Should be called only once, as it modifies the state of the
        /// terminal.
        pub fn init() -> std::io::Result<Self>
        {
            let mut stdout = MouseTerminal::from(std::io::stdout())
                .into_raw_mode()?;

            console::hide_cursor(&mut stdout)?;

            let buf_size = W as usize * H as usize;

            Ok(Self {
                chars: vec![' '; buf_size].into_boxed_slice(),
                styles: vec![Style::default().clean(); buf_size]
                    .into_boxed_slice(),
                cursor: Cursor { x: 0, y: 0, hidden: true },
                stdout,
            })
        }
    }

    impl<const W: u16, const H: u16> Backend for TermionFixed<W, H>
    {
        type Renderer<'r> = Buffer<'r>;
        type FlushError = std::io::Error;

        fn render<'a, 'r, F>(&'a mut self, ui: F)
        where
            F: FnOnce(&mut Self::Renderer<'r>),
            'a: 'r,
        {
            let mut buffer = Buffer::new(
                W,
                H,
                &mut self.chars,
                &mut self.styles,
                &mut self.cursor
            );
            buffer.clear();

            ui(&mut buffer);
        }

        fn flush(&mut self) -> Result<(), Self::FlushError>
        {
            let buffer = Buffer::new(
                W,
                H,
                &mut self.chars,
                &mut self.styles,
                &mut self.cursor
            );

            flush_buf(&mut self.stdout, &buffer)
        }
    }

    impl<const W: u16, const H: u16> Drop for TermionFixed<W, H> {
        fn drop(&mut self)
        {
            let _ = restore_terminal(&mut self.stdout, H);
        }
    }


    pub struct TermionDyn {
        last_width: u16,
        last_height: u16,
        last_flush_height: u16,
        chars: Vec<char>,
        styles: Vec<Style>,
        cursor: Cursor,
        stdout: RawTerminal<MouseTerminal<Stdout>>,
    }

    impl std::fmt::Debug for TermionDyn {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        {
            f.write_str("TermionDyn")
        }
    }

    impl TermionDyn {
        pub fn init() -> std::io::Result<Self>
        {
            let mut stdout = MouseTerminal::from(std::io::stdout())
                .into_raw_mode()?;

            console::hide_cursor(&mut stdout)?;

            let (width, height) = termion::terminal_size()?;

            let buf_size = width as usize * height as usize;

            Ok(Self {
                last_width: 0,
                last_height: 0,
                last_flush_height: 0,
                chars: vec![' '; buf_size],
                styles: vec![Style::default().clean(); buf_size],
                cursor: Cursor { x: 0, y: 0, hidden: true },
                stdout,
            })
        }
    }

    impl Backend for TermionDyn {
        type Renderer<'r> = Buffer<'r>;
        type FlushError = std::io::Error;

        fn render<'a, 'r, F>(&'a mut self, ui: F)
        where
            F: FnOnce(&mut Self::Renderer<'r>),
            'a: 'r
        {
            let (width, height) = termion::terminal_size()
                // TODO: log an error.
                .unwrap_or((self.last_width, self.last_height));

            let new_buf_size = width as usize * height as usize;
            // FIXME: sort of a memory leak.
            if new_buf_size > self.chars.len() {
                self.chars.resize(new_buf_size, ' ');
                self.styles.resize(new_buf_size, Style::default().clean());
            }

            self.last_width = width;
            self.last_height = height;

            let mut buffer = Buffer::new(
                self.last_width,
                self.last_height,
                &mut self.chars,
                &mut self.styles,
                &mut self.cursor
            );
            buffer.clear();

            ui(&mut buffer);
        }

        fn flush(&mut self) -> Result<(), Self::FlushError>
        {
            let buffer = Buffer::new(
                self.last_width,
                self.last_height,
                &mut self.chars,
                &mut self.styles,
                &mut self.cursor
            );

            flush_buf(&mut self.stdout, &buffer)?;

            self.last_flush_height = self.last_height;

            Ok(())
        }
    }

    impl Drop for TermionDyn {
        fn drop(&mut self)
        {
            let _ = restore_terminal(&mut self.stdout, self.last_flush_height);
        }
    }

    fn flush_buf<W: Write>(writer: &mut W, buffer: &Buffer)
        -> Result<(), std::io::Error>
    {
        for y in 0..buffer.height - 1 {
            write_line(writer, &buffer, y)?;
            console::write_str(writer, "\r\n")?;
        }

        write_line(writer, &buffer, buffer.height - 1)?;
        console::write_char(writer, '\r')?;
        console::move_cursor(writer, -(buffer.height as isize - 1), 0)?;

        // TODO: implement cursor with a real cursor.
        if !buffer.cursor.hidden {
            // Move the cursor to the its position.
            console::move_cursor(
                writer,
                buffer.cursor.y as isize,
                buffer.cursor.x as isize
            )?;
            // char printing
            console::add_text_style(writer, TextStyle::INVERT)?;
            console::write_char(
                writer,
                buffer.chars[offset!(
                    buffer.cursor.x,
                    buffer.cursor.y,
                    buffer.width
                )]
            )?;
            console::subtract_text_style(writer, TextStyle::INVERT)?;
            console::move_cursor(writer, 0, -1)?;
            // Move the cursor back to the top left of the screen.
            console::move_cursor(
                writer,
                -(buffer.cursor.y as isize),
                -(buffer.cursor.x as isize)
            )?;
        }

        writer.flush()?;

        Ok(())
    }

    fn write_line<W: Write>(writer: &mut W, buffer: &Buffer<'_>, y: u16)
        -> Result<(), std::io::Error>
    {
        let width = buffer.width as usize;
        let line_offset = offset!(0, y, width);
        let chars = &buffer.chars[line_offset..line_offset + width];
        let styles = &buffer.styles[line_offset..line_offset + width];

        let mut saved_ts = styles[0].text_style.unwrap_or_default();
        let mut saved_fg = styles[0].fg_color.unwrap_or_default();
        let mut saved_bg = styles[0].bg_color.unwrap_or_default();
        // The first char of every line is always set with colors and style.
        console::reset(writer)?;
        console::set_text_style(writer, saved_ts)?;
        console::set_fg_color(writer, saved_fg)?;
        console::set_bg_color(writer, saved_bg)?;
        console::write_char(writer, chars[0])?;

        for x in 1..width {
            let cur_style = &styles[x];
            let cur_char = &chars[x];

            let text_style = cur_style.text_style.unwrap_or_default();
            let fg_color = cur_style.fg_color.unwrap_or_default();
            let bg_color = cur_style.bg_color.unwrap_or_default();

            let ts_changed = saved_ts != text_style;
            if ts_changed {
                console::reset(writer)?;
                console::add_text_style(writer, text_style)?;
                saved_ts = text_style;
            }

            if saved_fg != fg_color || ts_changed {
                console::set_fg_color(writer, fg_color)?;
                saved_fg = fg_color;
            }
            if saved_bg != bg_color || ts_changed {
                console::set_bg_color(writer, bg_color)?;
                saved_bg = bg_color;
            }

            console::write_char(writer, *cur_char)?;
        }

        Ok(())
    }

    fn restore_terminal<W: Write>(stdout: &mut W, last_height: u16)
        -> std::io::Result<()>
    {
            console::set_fg_color(stdout, Color::Normal)?;
            console::set_bg_color(stdout, Color::Normal)?;
            console::set_text_style(stdout, TextStyle::NORMAL)?;
            for _row in 0..last_height {
                console::write_char(stdout, '\n')?;
            }
            console::show_cursor(stdout)
    }
}

mod console {
    use std::io::Write;

    use crate::style::{Color, TextStyle};

    #[inline]
    pub fn write_char<W: Write>(writer: &mut W, c: char)
        -> Result<(), std::io::Error>
    {
        write!(writer, "{}", c)
    }

    #[inline]
    pub fn write_str<W: Write>(writer: &mut W, s: &str)
        -> Result<(), std::io::Error>
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
    pub fn move_cursor<W: Write>(writer: &mut W, y: isize, x: isize)
        -> Result<(), std::io::Error>
    {
        // NOTE: it has to be checked for zero values, as supplying 0 to the
        // termion's cursor movement functions will result in the cursor being
        // moved by one position.

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
    pub fn reset<W: Write>(writer: &mut W) -> Result<(), std::io::Error>
    {
        write!(writer, "{}", termion::style::Reset)
    }

    #[inline]
    pub fn set_fg_color<W: Write>(writer: &mut W, color: Color)
        -> Result<(), std::io::Error>
    {
        use termion::color::*;
        use crate::style::Color;

        match color {
            Color::Normal       => write!(writer, "{}", Fg(Reset))?,
            Color::Black        => write!(writer, "{}", Fg(Black))?,
            Color::Red          => write!(writer, "{}", Fg(Red))?,
            Color::Green        => write!(writer, "{}", Fg(Green))?,
            Color::Yellow       => write!(writer, "{}", Fg(Yellow))?,
            Color::Blue         => write!(writer, "{}", Fg(Blue))?,
            Color::Magenta      => write!(writer, "{}", Fg(Magenta))?,
            Color::Cyan         => write!(writer, "{}", Fg(Cyan))?,
            Color::White        => write!(writer, "{}", Fg(White))?,
            Color::LightBlack   => write!(writer, "{}", Fg(LightBlack))?,
            Color::LightRed     => write!(writer, "{}", Fg(LightRed))?,
            Color::LightGreen   => write!(writer, "{}", Fg(LightGreen))?,
            Color::LightYellow  => write!(writer, "{}", Fg(LightYellow))?,
            Color::LightBlue    => write!(writer, "{}", Fg(LightBlue))?,
            Color::LightMagenta => write!(writer, "{}", Fg(LightMagenta))?,
            Color::LightCyan    => write!(writer, "{}", Fg(LightCyan))?,
            Color::LightWhite   => write!(writer, "{}", Fg(LightWhite))?,
            Color::Ansi(c)      => write!(writer, "{}", Fg(AnsiValue(c)))?,
            Color::Rgb(r, g, b) => write!(writer, "{}", Fg(Rgb(r, g, b)))?,
        }

        Ok(())
    }

    #[inline]
    pub fn set_bg_color<W: Write>(writer: &mut W, color: Color)
        -> Result<(), std::io::Error>
    {
        use termion::color::*;
        use crate::style::Color;

        match color {
            Color::Normal       => write!(writer, "{}", Bg(Reset))?,
            Color::Black        => write!(writer, "{}", Bg(Black))?,
            Color::Red          => write!(writer, "{}", Bg(Red))?,
            Color::Green        => write!(writer, "{}", Bg(Green))?,
            Color::Yellow       => write!(writer, "{}", Bg(Yellow))?,
            Color::Blue         => write!(writer, "{}", Bg(Blue))?,
            Color::Magenta      => write!(writer, "{}", Bg(Magenta))?,
            Color::Cyan         => write!(writer, "{}", Bg(Cyan))?,
            Color::White        => write!(writer, "{}", Bg(White))?,
            Color::LightBlack   => write!(writer, "{}", Bg(LightBlack))?,
            Color::LightRed     => write!(writer, "{}", Bg(LightRed))?,
            Color::LightGreen   => write!(writer, "{}", Bg(LightGreen))?,
            Color::LightYellow  => write!(writer, "{}", Bg(LightYellow))?,
            Color::LightBlue    => write!(writer, "{}", Bg(LightBlue))?,
            Color::LightMagenta => write!(writer, "{}", Bg(LightMagenta))?,
            Color::LightCyan    => write!(writer, "{}", Bg(LightCyan))?,
            Color::LightWhite   => write!(writer, "{}", Bg(LightWhite))?,
            Color::Ansi(c)      => write!(writer, "{}", Bg(AnsiValue(c)))?,
            Color::Rgb(r, g, b) => write!(writer, "{}", Bg(Rgb(r, g, b)))?,
        }

        Ok(())
    }

    #[inline]
    pub fn set_text_style<W: Write>(writer: &mut W, ts: TextStyle)
        -> Result<(), std::io::Error>
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
    pub fn add_text_style<W: Write>(writer: &mut W, ts: TextStyle)
        -> Result<(), std::io::Error>
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

    /// BUG: doesn't work for all attributes, mainly `TextStyle::BOLD`.
    ///
    /// This function should be completelty deprecated, as not all of these are
    /// supported universally. In such cases, the only way to undo a style is to
    /// do a full reset, which affects colors too.
    #[inline]
    pub fn subtract_text_style<W: Write>(writer: &mut W, ts: TextStyle)
        -> Result<(), std::io::Error>
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
