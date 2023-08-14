use crate::Pos;
use crate::style::WithStyle;
use super::{Widget, InnerWidget};
use crate::layout::Area;
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
    pub fn new(pos: Pos, width: u16) -> Self
    {
        let mut bar = Self {
            inner: InnerWidget::new(Area {
                x: pos.x,
                y: pos.y,
                width,
                height: 1
            }),
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

        self.inner.hfill(0, 0, self.theme.body, width as usize);
        self.inner.putc(0, 0, self.theme.beg);
        self.inner.putc(width - 1, 0, self.theme.end);
    }
}

impl Widget for HorizBar {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}

pub struct VertBar {
    inner: InnerWidget,
    theme: Theme,
}

impl VertBar {
    pub fn new(pos: Pos, height: u16) -> Self
    {
        let mut bar = Self {
            inner: InnerWidget::new(Area {
                x: pos.x,
                y: pos.y,
                width: 1,
                height,
            }),
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

        self.inner.vfill(0, 0, self.theme.body, height as usize);
        self.inner.putc(0, 0, self.theme.beg);
        self.inner.putc(0, height - 1, self.theme.end);
    }
}

impl Widget for VertBar {
    fn share_inner(&self) -> InnerWidget
    {
        self.inner.share()
    }
}
