use crate::widget::{InnerWidget, Widget};
use crate::layout::{self, Aligned, Justify, Align};
use crate::pos;

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
        {
            let inner = self.inner.borrow_mut();

            if inner.width < 2 || inner.height < 2 {
                return Err(());
            }

            if self.has_border {
                self.has_border = false;
            } else {
                self.has_border = true;
            }
        }
        if self.has_border {
            self.draw_border();
        } else {
            self.shift_content();
        }

        Ok(())
    }

    pub fn print(&mut self, y: u32, x: u32, line: &str)
    {
        let mut inner = self.inner.borrow_mut();

        let mut width = inner.width;
        let mut height = inner.height;

        if self.has_border {
            if width < 2 || height < 2 {
                return;
            }

            width -= 2;
            height -= 2;
        } else {
            if width < 1 || height < 1 {
                return;
            }
        }

        if x as usize >= width || y as usize >= height {
            return;
        }

        let ww = inner.width;

        for (i, c) in line.chars().enumerate() {
            if x as usize + i >= width as usize {
                break;
            }
            inner.buffer[pos![ww, y as usize, x as usize + i]] = c;
        }
    }

    pub fn putc(&mut self, mut y: u32, mut x: u32, c: char)
    {
        let mut inner = self.inner.borrow_mut();

        let mut width = inner.width;
        let mut height = inner.height;

        if self.has_border {
            if width < 2 || height < 2 {
                return;
            }

            y += 1;
            x += 1;
            width -= 2;
            height -= 2;
        } else {
            if width < 1 || height < 1 {
                return;
            }
        }

        if x as usize >= width || y as usize >= height {
            return;
        }

        let ww = inner.width;
        inner.buffer[pos![ww, y as usize, x as usize]] = c;
    }

    pub fn print_just(&mut self, j: Justify, line: &str)
    {
        match j {
            Justify::Left(row) => self.print(row, 0, line),
            Justify::HCentre(row) => {
                let x: usize;
                if line.len() >= self.inner_width() {
                    x = 0;
                } else {
                    x = (self.inner_width() - line.len()) / 2;
                }
                self.print(row, x as u32, line);
            },
            Justify::Right(row) => {
                let x: usize;
                if line.len() >= self.inner_width() {
                    x = 0;
                } else {
                    x = self.inner_width() - line.len();
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
            Justify::TopLeft => self.print_just(Justify::Left(0), line),
            Justify::TopCentre => self.print_just(Justify::HCentre(0), line),
            Justify::TopRight => self.print_just(Justify::Right(0), line),
            Justify::CentreLeft => self.print_just(Justify::VCentre(0), line),
            Justify::Centre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.print_just(Justify::HCentre(y as u32), line)
            },
            Justify::CentreRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                y /= 2;
                self.print_just(Justify::Right(y as u32), line)
            },
            Justify::BottomLeft => self.print_just(Justify::Bottom(0), line),
            Justify::BottomCentre => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.print_just(Justify::HCentre(y as u32), line)
            },
            Justify::BottomRight => {
                let mut y = self.inner_height();
                if y > 0 {
                    y -= 1;
                }
                self.print_just(Justify::Right(y as u32), line)
            },
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

        let start_y = inner.start_y as usize;
        let start_x = inner.start_x as usize;
        let height = inner.height;
        let width = inner.width;

        let sw = inner.width;

        if self.border_style.0 != '\0' {
            for i in 0..inner.width {
                inner.buffer[pos![sw, start_y, start_x + i]] = self.border_style.0;
                inner.buffer[pos![sw, start_y + height - 1, start_x + i]] = self.border_style.0;
            }
        }
        if self.border_style.1 != '\0' {
            for i in 0..inner.height {
                inner.buffer[pos![sw, start_y + i, start_x]] = self.border_style.1;
                inner.buffer[pos![sw, start_y + i, start_x + width - 1]] = self.border_style.1;
            }
        }
        if self.border_style.2 != '\0' {
            inner.buffer[pos![sw, start_y, start_x]] = self.border_style.2;
        }
        if self.border_style.3 != '\0' {
            inner.buffer[pos![sw, start_y, start_x + width - 1]] = self.border_style.3;
        }
        if self.border_style.4 != '\0' {
            inner.buffer[pos![sw, start_y + height - 1, start_x + width - 1]] = self.border_style.4;
        }
        if self.border_style.5 != '\0' {
            inner.buffer[pos![sw, start_y + height - 1, start_x]] = self.border_style.5;
        }
    }

    fn shift_content(&mut self)
    {
        let mut inner = self.inner.borrow_mut();
        let ww = inner.width;

        for y in 1..inner.height {
            for x in 1..inner.width {
                inner.buffer[pos![ww, y, x]] = inner.buffer[pos![ww, y - 1, x - 1]];
            }
        }
    }
}

impl Widget for Window {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }

    fn set_zindex(&mut self, index: u32)
    {
        self.inner.borrow_mut().z_index = index;
    }

    fn hide(&mut self)
    {
        self.inner.borrow_mut().hidden = true;
    }

    fn show(&mut self)
    {
        self.inner.borrow_mut().hidden = false;
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
}
