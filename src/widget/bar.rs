use crate::widget::{Widget, InnerWidget};
use crate::layout::{
    self,
    Aligned,
    Alignable,
    Align,
};
use crate::pos;

pub struct HorizBar {
    inner: InnerWidget,
    style: (char, char, char),
}

impl HorizBar {
    pub fn new(start_y: u32, start_x: u32, width: usize) -> Self
    {
        Self {
            inner: InnerWidget::new(start_y, start_x, 1, width),
            style: ('\0', '\0', '\0'),
        }
    }

    // left corner, bar, right corner.
    pub fn set_style(&mut self, style: (char, char, char))
    {
        self.style = style;
        self.redraw();
    }

    fn redraw(&mut self)
    {
        let style = self.style;

        let mut inner = self.inner.borrow_mut();
        let mut last_x = inner.width;
        if last_x > 0 {
            last_x -= 1;
        }
        let ww = inner.width;

        inner.buffer[pos![ww, 0, 0]] = style.0;
        inner.buffer[pos![ww, 0, last_x]] = style.2;
        for x in 1..last_x {
            inner.buffer[pos![ww, 0, x]] = style.1;
        }
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
    style: (char, char, char),
}

impl VertBar {
    pub fn new(start_y: u32, start_x: u32, height: usize) -> Self
    {
        Self {
            inner: InnerWidget::new(start_y, start_x, height, 1),
            style: ('\0', '\0', '\0'),
        }
    }

    // top corner, bar, bottom corner.
    pub fn set_style(&mut self, style: (char, char, char))
    {
        self.style = style;
        self.redraw();
    }

    fn redraw(&mut self)
    {
        let style = self.style;

        let mut inner = self.inner.borrow_mut();
        let mut last_y = inner.height;
        if last_y > 0 {
            last_y -= 1;
        }
        let ww = inner.width;

        inner.buffer[pos![ww, 0, 0]] = style.0;
        inner.buffer[pos![ww, last_y, 0]] = style.2;
        for y in 1..last_y {
            inner.buffer[pos![ww, y, 0]] = style.1;
        }
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
