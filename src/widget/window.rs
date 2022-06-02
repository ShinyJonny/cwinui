use crate::widget::{InnerWidget, Widget};
use crate::layout::{
    self,
    Aligned,
    Alignable,
    Justify,
    Align,
};
use crate::pos;
use crate::misc::SliceInChars;

pub struct Window {
    inner: InnerWidget,
    has_border: bool,
    border_style: (char, char, char, char, char, char),
}

impl Window {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self {
            inner: InnerWidget::new(start_y, start_x, height, width),
            has_border: false,
            border_style: ('\0', '\0', '\0', '\0', '\0', '\0'),
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

    // (horizontal bars, vertical bars, top-left corner, top-right corner, bottom-left corner,
    // bottom-right corner)
    pub fn set_border(&mut self, border: (char, char, char, char, char, char))
    {
        self.border_style = border;
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

    pub fn putc(&mut self, mut y: u32, mut x: u32, c: char)
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

    pub fn print(&mut self, mut y: u32, mut x: u32, line: &str)
    {
        let ch = self.content_height();
        let cw = self.content_width();
        if y >= ch as u32 || x >= cw as u32 {
            return;
        }

        let mut print_len = line.chars().count();
        if x as usize + print_len > cw {
            print_len = cw - x as usize;
        }

        if self.has_border {
            y += 1;
            x += 1;
        }
        self.inner.print(y, x, line.slice_in_chars(0, print_len));
    }

    pub fn printj(&mut self, j: Justify, line: &str)
    {
        let char_count = line.chars().count();

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
            Justify::TopLeft => self.printj(Justify::Left(0), line),
            Justify::TopCentre => self.printj(Justify::HCentre(0), line),
            Justify::TopRight => self.printj(Justify::Right(0), line),
            Justify::CentreLeft => self.printj(Justify::VCentre(0), line),
            Justify::Centre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.printj(Justify::HCentre(y as u32), line)
            },
            Justify::CentreRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.printj(Justify::Right(y as u32), line)
            },
            Justify::BottomLeft => self.printj(Justify::Bottom(0), line),
            Justify::BottomCentre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.printj(Justify::HCentre(y as u32), line)
            },
            Justify::BottomRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.printj(Justify::Right(y as u32), line)
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
        let mut inner = self.inner.borrow_mut();

        let mut height = inner.height;
        let mut width = inner.width;

        if height < 1 {
            height = 1;
        }
        if width < 1 {
            width = 1;
        }

        let w = inner.width;

        if self.border_style.0 != '\0' {
            for i in 0..inner.width {
                inner.buffer[pos![w, 0, 0 + i]] = self.border_style.0;
                inner.buffer[pos![w, 0 + height - 1, 0 + i]] = self.border_style.0;
            }
        }
        if self.border_style.1 != '\0' {
            for i in 0..inner.height {
                inner.buffer[pos![w, 0 + i, 0]] = self.border_style.1;
                inner.buffer[pos![w, 0 + i, 0 + width - 1]] = self.border_style.1;
            }
        }
        if self.border_style.2 != '\0' {
            inner.buffer[pos![w, 0, 0]] = self.border_style.2;
        }
        if self.border_style.3 != '\0' {
            inner.buffer[pos![w, 0, 0 + width - 1]] = self.border_style.3;
        }
        if self.border_style.4 != '\0' {
            inner.buffer[pos![w, 0 + height - 1, 0 + width - 1]] = self.border_style.4;
        }
        if self.border_style.5 != '\0' {
            inner.buffer[pos![w, 0 + height - 1, 0]] = self.border_style.5;
        }
    }

    fn clear_border(&mut self)
    {
        let mut inner = self.inner.borrow_mut();

        let mut height = inner.height;
        let mut width = inner.width;

        if height < 1 {
            height = 1;
        }
        if width < 1 {
            width = 1;
        }

        let w = inner.width;

        for i in 0..inner.width {
            inner.buffer[pos![w, 0, 0 + i]] = '\0';
            inner.buffer[pos![w, 0 + height - 1, 0 + i]] = '\0';
        }
        for i in 0..inner.height {
            inner.buffer[pos![w, 0 + i, 0]] = '\0';
            inner.buffer[pos![w, 0 + i, 0 + width - 1]] = '\0';
        }
    }

    fn shift_content_in(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width;

        for y in 1..inner.height {
            for x in 1..inner.width {
                inner.buffer[pos![w, y, x]] = inner.buffer[pos![w, y - 1, x - 1]];
            }
        }
    }

    fn shift_content_out(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let w = inner.width;

        for y in 1..inner.height {
            for x in 1..inner.width {
                inner.buffer[pos![w, y - 1, x - 1]] = inner.buffer[pos![w, y, x]];
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
