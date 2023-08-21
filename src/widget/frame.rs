use crate::{style::StyledChar, screen::Buffer, Pos, Area};

use super::Widget;

pub struct Theme {
    pub top_left: StyledChar,
    pub top_right: StyledChar,
    pub bottom_right: StyledChar,
    pub bottom_left: StyledChar,
    pub top: StyledChar,
    pub right: StyledChar,
    pub bottom: StyledChar,
    pub left: StyledChar,
}

impl Default for Theme {
    fn default() -> Self
    {
        Self {
            top_left: '#'.into(),
            top_right: '#'.into(),
            bottom_right: '#'.into(),
            bottom_left: '#'.into(),
            top: '#'.into(),
            right: '#'.into(),
            bottom: '#'.into(),
            left: '#'.into(),
        }
    }
}

pub struct Frame<T: Widget> {
    pub theme: Theme,
    pub inner: T,
}

impl<T: Widget> Frame<T> {
    pub fn new(inner: T) -> Self
    {
        Self {
            inner,
            theme: Theme::default(),
        }
    }

    pub fn theme<C>(
        mut self,
        top_left: C,
        top_right: C,
        bottom_right: C,
        bottom_left: C,
        top: C,
        right: C,
        bottom: C,
        left: C,
    ) -> Self
    where
        C: Into<StyledChar>
    {
        self.theme = Theme {
            top_left: top_left.into(),
            top_right: top_right.into(),
            bottom_right: bottom_right.into(),
            bottom_left: bottom_left.into(),
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        };

        self
    }
}

impl<T: Widget> Widget for Frame<T> {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Sides

        let Pos { x, y } = area.top_left();
        buf.hfill(x, y, self.theme.top, area.width as usize);

        let Pos { x, y } = area.top_right().sub_x(1);
        buf.vfill(x, y, self.theme.right, area.width as usize);

        let Pos { x, y } = area.bottom_left().sub_y(1);
        buf.hfill(x, y, self.theme.bottom, area.width as usize);

        let Pos { x, y } = area.top_left();
        buf.vfill(x, y, self.theme.left, area.height as usize);

        // Corners

        let Pos { x, y } = area.top_left();
        buf.putc(x, y, self.theme.top_left);

        let Pos { x, y } = area.top_right().sub_x(1);
        buf.putc(x, y, self.theme.top_right);

        let Pos { x, y } = area.bottom_right() - Pos { x:1, y: 1 };
        buf.putc(x, y, self.theme.bottom_right);

        let Pos { x, y } = area.bottom_left().sub_y(1);
        buf.putc(x, y, self.theme.bottom_left);

        // Inner

        let inner_area = if area.width >= 2 && area.height >= 2
            { area.inset(1) }
            else {
                Area {
                    x: area.x + 1,
                    y: area.y + 1,
                    width: 0,
                    height: 0
                }
            };

        self.inner.render(buf, inner_area);
    }
}
