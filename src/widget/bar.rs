use crate::Pos;
use crate::screen::Buffer;
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

impl Widget for HorizBar {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let Pos {x, y} = area.top_left();
        buf.vfill(x, y, self.theme.body, area.width as usize);
        buf.putc(x, y, self.theme.beg);
        let Pos {x, y} = area.top_right().sub_x(1);
        buf.putc(x, y, self.theme.end);
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

impl Widget for VertBar {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let Pos {x, y} = area.top_left();
        buf.hfill(x, y, self.theme.body, area.height as usize);
        buf.putc(x, y, self.theme.beg);
        let Pos {x, y} = area.bottom_left().sub_y(1);
        buf.putc(x, y, self.theme.end);
    }
}
