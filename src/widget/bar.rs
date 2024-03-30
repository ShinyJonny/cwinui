use crate::paint::Paint;
use crate::style::WithStyle;
use super::Widget;
use crate::layout::Area;
use crate::style::StyledChar;

pub struct Theme {
    pub beg: StyledChar,
    pub end: StyledChar,
    pub body: StyledChar,
}

impl Default for Theme {
    fn default() -> Self
    {
        Self {
            beg: '\0'.styled(),
            end: '\0'.styled(),
            body: '\0'.styled(),
        }
    }
}

pub struct HorizBar {
    pub theme: Theme,
}

impl HorizBar {
    pub fn new() -> Self
    {
        Self {
            theme: Theme::default(),
        }
    }

    #[inline]
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

        self
    }
}

impl<P: Paint> Widget<P> for HorizBar {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() {
            return;
        }

        let top_left = area.top_left();
        buf.vfill(top_left, self.theme.body, area.width as usize);
        buf.putc_abs(top_left, self.theme.beg);
        buf.putc_abs(area.top_right().sub_x(1), self.theme.end);
    }
}

pub struct VertBar {
    pub theme: Theme,
}

impl VertBar {
    pub fn new() -> Self
    {
        Self {
            theme: Theme {
                beg: '0'.styled(),
                end: '0'.styled(),
                body: '0'.styled(),
            },
        }
    }

    #[inline]
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

        self
    }
}

impl<P: Paint> Widget<P> for VertBar {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() {
            return;
        }

        let top_left = area.top_left();
        buf.hfill(top_left, self.theme.body, area.height as usize);
        buf.putc_abs(top_left, self.theme.beg);
        buf.putc_abs(area.bottom_left().sub_y(1), self.theme.end);
    }
}
