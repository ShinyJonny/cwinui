use crate::style::WithStyle;
use super::{InnerWidget, Widget};
use crate::layout::{Justify, Area};
use crate::util::offset;
use crate::misc::SliceInChars;
use crate::style::{StyledChar, StyledStr};

struct Theme {
    top_bar:             StyledChar,
    right_bar:           StyledChar,
    bottom_bar:          StyledChar,
    left_bar:            StyledChar,
    topleft_corner:      StyledChar,
    topright_corner:     StyledChar,
    bottomright_corner:  StyledChar,
    bottomleft_corner:   StyledChar,
}

pub struct Window {
    inner: InnerWidget,
    has_border: bool,
    theme: Theme,
}

impl Window {
    pub fn new(area: Area) -> Self
    {
        Self {
            inner: InnerWidget::new(area),
            has_border: false,
            // TODO: add border style for each side.
            theme: Theme {
                top_bar:            '\0'.styled(),
                right_bar:          '\0'.styled(),
                bottom_bar:         '\0'.styled(),
                left_bar:           '\0'.styled(),
                topleft_corner:     '\0'.styled(),
                topright_corner:    '\0'.styled(),
                bottomright_corner: '\0'.styled(),
                bottomleft_corner:  '\0'.styled(),
            },
        }
    }

    pub fn content_area(&self) -> Area
    {
        let inner = self.inner.borrow();

        let area = Area {
            x: inner.start_x,
            y: inner.start_y,
            width: inner.width,
            height: inner.height,
        };

        if self.has_border {
            area.inset(1)
        } else {
            area
        }
    }

    pub fn set_theme<C>(
        &mut self,
        top_bar: C,
        right_bar: C,
        bottom_bar: C,
        left_bar: C,
        topleft_corner: C,
        topright_corner: C,
        bottomright_corner: C,
        bottomleft_corner: C
    )
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            top_bar: top_bar.into(),
            right_bar: right_bar.into(),
            bottom_bar: bottom_bar.into(),
            left_bar: left_bar.into(),
            topleft_corner: topleft_corner.into(),
            topright_corner: topright_corner.into(),
            bottomright_corner: bottomright_corner.into(),
            bottomleft_corner: bottomleft_corner.into(),
        };
        if self.has_border {
            self.draw_border();
        }
    }

    pub fn toggle_border(&mut self) -> Result<(), ()>
    {
        let inner = self.inner.borrow_mut();
        if !self.has_border && (inner.width < 2 || inner.height < 2) {
            return Err(());
        }
        drop(inner);

        if self.has_border {
            self.has_border = false;
            self.clear_border();
            self.shift_content_out();
        } else {
            self.has_border = true;
            self.shift_content_in();
            self.draw_border();
        }

        Ok(())
    }

    pub fn putc<C>(&mut self, mut x: u16, mut y: u16, c: C)
    where
        C: Into<StyledChar>
    {
        let content_area = self.content_area();
        let cw = content_area.width;
        let ch = content_area.height;
        if x >= cw || y >= ch {
            return;
        }

        if self.has_border {
            y += 1;
            x += 1;
        }
        self.inner.putc(x, y, c);
    }

    pub fn print<'s, T>(&mut self, mut x: u16, mut y: u16, line: T)
    where
        T: Into<StyledStr<'s>>
    {
        // TODO: support printing with newlines (and other non-standard whitespace).
        // TODO: check for variable-length characters.

        let mut line = line.into();

        let content_area = self.content_area();
        let cw = content_area.width;
        let ch = content_area.height;
        if x >= cw || y >= ch {
            return;
        }

        let mut print_len = line.content.chars().count();
        if x as usize + print_len > cw as usize {
            print_len = cw as usize - x as usize;
        }

        if self.has_border {
            x += 1;
            y += 1;
        }

        if print_len < line.content.chars().count() {
            // FIXME: use native slicing API.
            line = StyledStr {
                content: line.content.slice_in_chars(0, print_len),
                style: line.style,
            }
        }

        self.inner.print(x, y, line);
    }

    pub fn printj<'s, T>(&mut self, line: T, j: Justify)
    where
        T: Into<StyledStr<'s>>
    {
        // TODO: support printing with newlines (and other non-standard whitespace).
        // FIXME: check for variable-length characters.

        let line = line.into();

        let char_count = line.content.chars().count();
        let content = self.content_area();

        match j {
            Justify::Left(row)    => self.print(0, row, line),
            Justify::HCentre(row) => {
                let x = if char_count >= content.width as usize
                    { 0 }
                    else { (content.width as usize - char_count) as u16 / 2 };
                self.print(x, row, line);
            },
            Justify::Right(row) => {
                let x = if char_count >= content.width as usize
                    { 0 }
                    else { (content.width as usize - char_count) as u16 };
                self.print(x, row, line);
            },
            Justify::Top(col)     => self.print(col, 0, line),
            Justify::VCentre(col) => {
                let y = content.height.saturating_sub(1) / 2;
                self.print(col, y, line)
            },
            Justify::Bottom(col) => {
                let y = content.height.saturating_sub(1);
                self.print(col, y, line)
            },
            Justify::TopLeft    => self.printj(line, Justify::Left(0)),
            Justify::TopCentre  => self.printj(line, Justify::HCentre(0)),
            Justify::TopRight   => self.printj(line, Justify::Right(0)),
            Justify::CentreLeft => self.printj(line, Justify::VCentre(0)),
            Justify::Centre     => {
                let y = content.height.saturating_sub(1) / 2;
                self.printj(line, Justify::HCentre(y))
            },
            Justify::CentreRight => {
                let y = content.height.saturating_sub(1) / 2;
                self.printj(line, Justify::Right(y))
            },
            Justify::BottomLeft   => self.printj(line, Justify::Bottom(0)),
            Justify::BottomCentre => {
                let y = content.height.saturating_sub(1);
                self.printj(line, Justify::HCentre(y))
            },
            Justify::BottomRight => {
                let y = content.height.saturating_sub(1);
                self.printj(line, Justify::Right(y))
            },
        }
    }

    pub fn clearln(&mut self, y: u16)
    {
        let cw = self.content_area().width;
        if y >= cw {
            return;
        }

        for x in 0..cw {
            self.putc(x, y, '\0');
        }
    }

    pub fn clear(&mut self)
    {
        self.inner.clear();
        if self.has_border {
            self.draw_border();
        }
    }

    fn draw_border(&mut self)
    {
        let inner = self.inner.borrow_mut();

        let height = inner.height;
        let width = inner.width;

        drop(inner);

        if height < 1 || width < 1 {
            return;
        }

        // Top and bottom edges.
        self.inner.hfill(0, 0, self.theme.top_bar, width as usize);
        self.inner.hfill(0, height - 1, self.theme.bottom_bar, width as usize);
        // Right and left edges.
        self.inner.vfill(0, 0, self.theme.left_bar, height as usize);
        self.inner.vfill(width - 1, 0, self.theme.right_bar, height as usize);
        // Corners.
        self.inner.putc(0, 0, self.theme.topleft_corner);
        self.inner.putc(0 + width - 1, 0, self.theme.topright_corner);
        self.inner.putc(0 + width - 1, 0 + height - 1, self.theme.bottomright_corner);
        self.inner.putc(0, 0 + height - 1, self.theme.bottomleft_corner);
    }

    fn clear_border(&mut self)
    {
        let inner = self.inner.borrow_mut();

        let height = inner.height;
        let width = inner.width;

        drop(inner);

        if height < 1 || width < 1 {
            return;
        }

        // Top and bottom edges.
        self.inner.hfill(0, 0, '\0', width as usize);
        self.inner.hfill(0, height - 1, '\0', width as usize);
        // Right and left edges.
        self.inner.vfill(0, 0, '\0', height as usize);
        self.inner.vfill(width - 1, 0, '\0', height as usize);
    }

    fn shift_content_in(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width as usize;

        for y in 1..inner.height as usize {
            for x in 1..inner.width as usize {
                //FIXME: implement this through APIs.
                inner.buffer[offset![x, y, w]]
                    = inner.buffer[offset![x - 1, y - 1, w]];
                inner.style_buffer[offset![x, y, w]]
                    = inner.style_buffer[offset![x - 1, y - 1, w]];
            }
        }
    }

    fn shift_content_out(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width as usize;

        for y in 1..inner.height as usize {
            for x in 1..inner.width as usize {
                //FIXME: implement this through APIs.
                inner.buffer[offset![x - 1, y - 1, w]]
                    = inner.buffer[offset![x, y, w]];
                inner.style_buffer[offset![x - 1, y - 1, w]]
                    = inner.style_buffer[offset![x, y, w]];
            }
        }
    }
}

impl Widget for Window {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}
