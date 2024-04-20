use crate::style::StyledChar;
use crate::widget::Paint;
use crate::{Pos, Area,};

use super::{Widget, Dummy};

/// Configuration options for theming [Frame].
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Frame<T = Dummy> {
    pub theme: Theme,
    pub inner: T,
}

impl<T> Frame<T> {
    /// Creates a new `Frame` containing `inner`.
    pub fn new(inner: T) -> Self
    {
        Self {
            inner,
            theme: Theme::default(),
        }
    }

    /// Adjusts the theme of the `Frame`.
    #[inline]
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

impl<T: Widget<P>, P: Paint> Widget<P> for Frame<T> {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_collapsed() {
            return;
        }

        // Sides

        let top_left = area.top_left();
        let top_right = area.top_right().sub_x(1);
        let bottom_left = area.bottom_left().sub_y(1);
        let bottom_right = area.bottom_right() - Pos { x: 1, y: 1 };

        buf.hfill(top_left, self.theme.top, area.width as usize);
        buf.vfill(top_right, self.theme.right, area.width as usize);
        buf.hfill(bottom_left, self.theme.bottom, area.width as usize);
        buf.vfill(top_left, self.theme.left, area.height as usize);

        // Corners

        buf.putc_abs(top_left, self.theme.top_left);
        buf.putc_abs(top_right, self.theme.top_right);
        buf.putc_abs(bottom_left, self.theme.bottom_left);
        buf.putc_abs(bottom_right, self.theme.bottom_right);

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
