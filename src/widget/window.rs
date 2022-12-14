use crate::style::WithStyle;
use super::{InnerWidget, Widget};
use crate::layout::{
    self,
    Aligned,
    Alignable,
    Justify,
    Align,
};
use crate::pos;
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
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self {
            inner: InnerWidget::new(start_y, start_x, height, width),
            has_border: false,
            // TODO: add border style for each side.
            theme: Theme {
                top_bar:             '\0'.styled(),
                right_bar:           '\0'.styled(),
                bottom_bar:          '\0'.styled(),
                left_bar:            '\0'.styled(),
                topleft_corner:      '\0'.styled(),
                topright_corner:     '\0'.styled(),
                bottomright_corner:  '\0'.styled(),
                bottomleft_corner:   '\0'.styled(),
            },
        }
    }

    pub fn content_width(&self) -> usize
    {
        let inner = self.inner.borrow();

        if self.has_border {
            inner.width - 2
        } else {
            inner.width
        }
    }

    pub fn content_height(&self) -> usize
    {
        let inner = self.inner.borrow();

        if self.has_border {
            inner.height - 2
        } else {
            inner.height
        }
    }

    pub fn content_yx(&self) -> (u32, u32)
    {
        let inner = self.inner.borrow();

        if self.has_border {
            (inner.start_y + 1, inner.start_x + 1)
        } else {
            (inner.start_y, inner.start_x)
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

    pub fn putc<C>(&mut self, mut y: u32, mut x: u32, c: C)
    where
        C: Into<StyledChar>
    {
        let ch = self.content_height();
        let cw = self.content_width();
        if y >= ch as u32 || x >= cw as u32 {
            return;
        }

        if self.has_border {
            y += 1;
            x += 1;
        }
        self.inner.putc(y, x, c);
    }

    pub fn print<'s, T>(&mut self, mut y: u32, mut x: u32, line: T)
    where
        T: Into<StyledStr<'s>>
    {
        // TODO: support printing with newlines (and other non-standard whitespace).
        // TODO: check for variable-length characters.

        let mut line = line.into();

        let ch = self.content_height();
        let cw = self.content_width();
        if y >= ch as u32 || x >= cw as u32 {
            return;
        }

        let mut print_len = line.content.chars().count();
        if x as usize + print_len > cw {
            print_len = cw - x as usize;
        }

        if self.has_border {
            y += 1;
            x += 1;
        }

        if print_len < line.content.chars().count() {
            // FIXME: use native slicing API.
            line = StyledStr {
                content: line.content.slice_in_chars(0, print_len),
                style: line.style,
            }
        }

        self.inner.print(y, x, line);
    }

    pub fn printj<'s, T>(&mut self, line: T, j: Justify)
    where
        T: Into<StyledStr<'s>>
    {
        // TODO: support printing with newlines (and other non-standard whitespace).
        // FIXME: check for variable-length characters.

        let line = line.into();

        let char_count = line.content.chars().count();

        match j {
            Justify::Left(row) => self.print(row, 0, line),
            Justify::HCentre(row) => {
                let x: usize;
                if char_count >= self.inner_width() {
                    x = 0;
                } else {
                    x = (self.inner_width() - char_count) / 2;
                }
                self.print(row, x as u32, line);
            },
            Justify::Right(row) => {
                let x: usize;
                if char_count >= self.inner_width() {
                    x = 0;
                } else {
                    x = self.inner_width() - char_count;
                }
                self.print(row, x as u32, line);
            },
            Justify::Top(col) => self.print(0, col, line),
            Justify::VCentre(col) => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.print(y as u32, col, line)
            },
            Justify::Bottom(col) => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.print(y as u32, col, line)
            },
            Justify::TopLeft => self.printj(line, Justify::Left(0)),
            Justify::TopCentre => self.printj(line, Justify::HCentre(0)),
            Justify::TopRight => self.printj(line, Justify::Right(0)),
            Justify::CentreLeft => self.printj(line, Justify::VCentre(0)),
            Justify::Centre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.printj(line, Justify::HCentre(y as u32))
            },
            Justify::CentreRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.printj(line, Justify::Right(y as u32))
            },
            Justify::BottomLeft => self.printj(line, Justify::Bottom(0)),
            Justify::BottomCentre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.printj(line, Justify::HCentre(y as u32))
            },
            Justify::BottomRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.printj(line, Justify::Right(y as u32))
            },
        }
    }

    pub fn clearln(&mut self, y: usize)
    {
        let cw = self.content_width();
        if y >= cw {
            return;
        }

        for x in 0..cw {
            self.putc(y as u32, x as u32, '\0');
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

        let height = inner.height as u32;
        let width = inner.width as u32;

        drop(inner);

        if height < 1 || width < 1 {
            return;
        }

        // Top and bottom edges.
        self.inner.hfill(0, 0, self.theme.top_bar, width as usize);
        self.inner.hfill(height - 1, 0, self.theme.bottom_bar, width as usize);
        // Right and left edges.
        self.inner.vfill(0, 0, self.theme.left_bar, height as usize);
        self.inner.vfill(0, width - 1, self.theme.right_bar, height as usize);
        // Corners.
        self.inner.putc(0, 0, self.theme.topleft_corner);
        self.inner.putc(0, 0 + width - 1, self.theme.topright_corner);
        self.inner.putc(0 + height - 1, 0 + width - 1, self.theme.bottomright_corner);
        self.inner.putc(0 + height - 1, 0, self.theme.bottomleft_corner);
    }

    fn clear_border(&mut self)
    {
        let inner = self.inner.borrow_mut();

        let height = inner.height as u32;
        let width = inner.width as u32;

        drop(inner);

        if height < 1 || width < 1 {
            return;
        }

        // Top and bottom edges.
        self.inner.hfill(0, 0, '\0', width as usize);
        self.inner.hfill(height - 1, 0, '\0', width as usize);
        // Right and left edges.
        self.inner.vfill(0, 0, '\0', height as usize);
        self.inner.vfill(0, width - 1, '\0', height as usize);
    }

    fn shift_content_in(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width;

        for y in 1..inner.height {
            for x in 1..inner.width {
                //FIXME: implement this through APIs.
                inner.buffer[pos![w, y, x]] = inner.buffer[pos![w, y - 1, x - 1]];
                inner.style_buffer[pos![w, y, x]] = inner.style_buffer[pos![w, y - 1, x - 1]];
            }
        }
    }

    fn shift_content_out(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width;

        for y in 1..inner.height {
            for x in 1..inner.width {
                //FIXME: implement this through APIs.
                inner.buffer[pos![w, y - 1, x - 1]] = inner.buffer[pos![w, y, x]];
                inner.style_buffer[pos![w, y - 1, x - 1]] = inner.style_buffer[pos![w, y, x]];
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

impl Aligned for Window {
    fn inner_width(&self) -> usize
    {
        self.content_width()
    }

    fn inner_height(&self) -> usize
    {
        self.content_height()
    }

    fn inner_start_yx(&self) -> (u32, u32)
    {
        self.content_yx()
    }

    fn outer_width(&self) -> usize
    {
        self.inner.borrow().width
    }

    fn outer_height(&self) -> usize
    {
        self.inner.borrow().height
    }

    fn outer_start_yx(&self) -> (u32, u32)
    {
        let inner = self.inner.borrow();
        (inner.start_y, inner.start_x)
    }

    fn centre(&self) -> (u32, u32)
    {
        let inner = self.inner.borrow();

        let (mut centre_y, mut centre_x) = (
            inner.start_y + inner.height as u32 / 2,
            inner.start_x + inner.width as u32 / 2
        );
        if centre_y > 0 {
            centre_y -= 1;
        }
        if centre_x > 0 {
            centre_x -= 1;
        }

        (centre_y, centre_x)
    }
}

impl Alignable for Window {
    fn align_centres<T: Aligned>(&mut self, anchor: &T)
    {
        let (acy, acx) = anchor.centre();
        let (scy, scx) = self.centre();

        let acy = acy as i64;
        let acx = acx as i64;
        let scy = scy as i64;
        let scx = scx as i64;

        let mut inner = self.inner.borrow_mut();
        inner.start_y = (inner.start_y as i64 + (acy - scy)) as u32;
        inner.start_x = (inner.start_x as i64 + (acx - scx)) as u32;
    }

    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let mut inner = self.inner.borrow_mut();

        let (ay, ax) = anchor.inner_start_yx();
        let aheight = anchor.inner_height();
        let awidth = anchor.inner_width();
        let sheight = inner.height;
        let swidth = inner.width;

        let (new_y, new_x) = layout::align(
            a,
            sheight, swidth,
            ay, ax, aheight, awidth
        );

        inner.start_y = new_y;
        inner.start_x = new_x;
    }

    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let mut inner = self.inner.borrow_mut();

        let (ay, ax) = anchor.outer_start_yx();
        let aheight = anchor.outer_height();
        let awidth = anchor.outer_width();
        let sheight = inner.height;
        let swidth = inner.width;

        let (new_y, new_x) = layout::align(
            a,
            sheight, swidth,
            ay, ax, aheight, awidth
        );

        inner.start_y = new_y;
        inner.start_x = new_x;
    }

    fn adjust_pos(&mut self, y: i32, x: i32)
    {
        let mut inner = self.inner.borrow_mut();
        let new_y = inner.start_y as i32 + y;
        let new_x = inner.start_x as i32 + x;

        if new_y < 0 || new_x < 0 {
            panic!("position adjustment is out of bounds");
        }

        inner.start_y = new_y as u32;
        inner.start_x = new_x as u32;
    }

    fn change_pos(&mut self, y: u32, x: u32)
    {
        let mut inner = self.inner.borrow_mut();
        inner.start_y = y;
        inner.start_x = x;
    }
}
