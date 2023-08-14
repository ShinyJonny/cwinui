use crate::style::WithStyle;
use super::{Widget, InnerWidget};
use crate::layout::{
    self,
    Aligned,
    Alignable,
    Align,
};
use crate::style::StyledChar;

struct Theme {
    beg: StyledChar,
    end: StyledChar,
    body: StyledChar,
}

pub struct HorizBar {
    inner: InnerWidget,
    theme: Theme,
}

impl HorizBar {
    pub fn new(start_y: u32, start_x: u32, width: usize) -> Self
    {
        let mut bar = Self {
            inner: InnerWidget::new(start_y, start_x, 1, width),
            theme: Theme {
                beg: '\0'.styled(),
                end: '\0'.styled(),
                body: '\0'.styled(),
            },
        };
        bar.redraw();

        bar
    }

    pub fn theme<C>(
        mut self,
        beg: C,
        end: C,
        body: C,
    ) -> Self
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            beg: beg.into(),
            end: end.into(),
            body: body.into(),
        };
        self.redraw();

        self
    }

    pub fn set_theme<C>(
        &mut self,
        beg: C,
        end: C,
        body: C,
    )
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            beg: beg.into(),
            end: end.into(),
            body: body.into(),
        };
        self.redraw();
    }

    fn redraw(&mut self)
    {
        let width = self.inner.borrow().width;

        self.inner.hfill(0, 0, self.theme.body, width);
        self.inner.putc(0, 0, self.theme.beg);
        self.inner.putc(width as u32 - 1, 0, self.theme.end);
    }
}

impl Widget for HorizBar {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}

impl Aligned for HorizBar {
    fn inner_width(&self) -> usize
    {
        self.outer_width()
    }

    fn inner_height(&self) -> usize
    {
        self.outer_height()
    }

    fn inner_start_yx(&self) -> (u32, u32)
    {
        self.outer_start_yx()
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

impl Alignable for HorizBar {
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
        self.align_to_outer(anchor, a);
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

pub struct VertBar {
    inner: InnerWidget,
    theme: Theme,
}

impl VertBar {
    pub fn new(start_y: u32, start_x: u32, height: usize) -> Self
    {
        let mut bar = Self {
            inner: InnerWidget::new(start_y, start_x, height, 1),
            theme: Theme {
                beg: '0'.styled(),
                end: '0'.styled(),
                body: '0'.styled(),
            },
        };
        bar.redraw();

        bar
    }

    pub fn theme<C>(
        mut self,
        beg: C,
        end: C,
        body: C,
    ) -> Self
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            beg: beg.into(),
            end: end.into(),
            body: body.into(),
        };
        self.redraw();

        self
    }

    pub fn set_theme<C>(
        &mut self,
        beg: C,
        end: C,
        body: C,
    )
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            beg: beg.into(),
            end: end.into(),
            body: body.into(),
        };
        self.redraw();
    }

    fn redraw(&mut self)
    {
        let height = self.inner.borrow_mut().height;

        self.inner.vfill(0, 0, self.theme.body, height);
        self.inner.putc(0, 0, self.theme.beg);
        self.inner.putc(0, height as u32 - 1, self.theme.end);
    }
}

impl Widget for VertBar {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}

impl Aligned for VertBar {
    fn inner_width(&self) -> usize
    {
        self.outer_width()
    }

    fn inner_height(&self) -> usize
    {
        self.outer_height()
    }

    fn inner_start_yx(&self) -> (u32, u32)
    {
        self.outer_start_yx()
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

impl Alignable for VertBar {
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
        self.align_to_outer(anchor, a);
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
