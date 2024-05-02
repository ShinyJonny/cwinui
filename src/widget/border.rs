use crate::layout::{Proportional, Proportions};
use crate::style::{Style, StyledChar};
use crate::{Area, Dim, Pos};

use super::{Draw, Paint};

/// Configuration options for theming [`Border`].
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

/// Adds a border around the contained widget.
#[derive(Debug, Clone)]
pub struct Border<T> {
    pub theme: Theme,
    pub inner: T,
}

impl<T> Border<T> {
    /// Wraps `inner` in a `Border`.
    pub const fn new(inner: T) -> Self
    {
        Self {
            inner,
            theme: Theme::default(),
        }
    }

    /// Adjusts the theme of the `Border`.
    #[inline]
    pub const fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

        self
    }
}

impl<T: Draw<P>, P: Paint> Draw<P> for Border<T> {
    fn draw(&self, buf: &mut P, area: Area)
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

        self.inner.draw(buf, inner_area);
    }
}

impl<T> Proportional for Border<T>
where
    T: Proportional
{
    fn proportions(&self) -> Proportions
    {
        self.inner.proportions()
            .add(Proportions::fixed(Dim { width: 2, height: 2 }))
    }
}
