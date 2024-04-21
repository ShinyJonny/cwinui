use crate::style::{Style, StyledChar};
use crate::widget::Paint;
use crate::{Pos, Area,};

use super::{Widget, Dummy};

/// Configuration options for theming [`Frame`].
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

impl Theme {
    /// Const version of `Default::default`.
    pub const fn default() -> Self
    {
        let c = StyledChar { content: '#', style: Style::default() };
        Self {
            top_left: c,
            top_right: c,
            bottom_right: c,
            bottom_left: c,
            top: c,
            right: c,
            bottom: c,
            left: c,
        }
    }
}

impl Default for Theme {
    fn default() -> Self
    {
        Self::default()
    }
}

/// Adds border around the contained widget.
#[derive(Debug, Clone)]
pub struct Frame<T = Dummy> {
    pub theme: Theme,
    pub inner: T,
}

impl<T> Frame<T> {
    /// Creates a new `Frame` containing `inner`.
    pub const fn new(inner: T) -> Self
    {
        Self {
            inner,
            theme: Theme::default(),
        }
    }

    /// Adjusts the theme of the `Frame`.
    #[inline]
    pub const fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

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
        buf.hfill(bottom_left, self.theme.bottom, area.width as usize);
        buf.vfill(top_left, self.theme.left, area.height as usize);
        buf.vfill(top_right, self.theme.right, area.height as usize);

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
