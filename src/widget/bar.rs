use crate::widget::Render;
use crate::style::Style;
use super::Draw;
use crate::layout::{Area, Proportional, Proportions};
use crate::style::StyledChar;

/// Configuration options for theming [`HorizBar`] and [`VertBar`].
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub beg: StyledChar,
    pub end: StyledChar,
    pub body: StyledChar,
}

impl Theme {
    /// Const version of `Default::default`.
    #[inline]
    pub const fn default() -> Self
    {
        let c = StyledChar { content: '\0', style: Style::default() };
        Self {
            beg: c,
            end: c,
            body: c,
        }
    }
}

impl Default for Theme {
    fn default() -> Self
    {
        Self::default()
    }
}

/// Draws a horizontal bar starting at the top-left corner of the paint area
/// and spanning the full width of the paint area.
#[derive(Debug, Clone, Default)]
pub struct HorizBar {
    pub theme: Theme,
}

impl HorizBar {
    /// Creates a new `HorizBar`.
    #[inline]
    pub const fn new() -> Self
    {
        Self {
            theme: Theme::default(),
        }
    }

    /// Adjusts the theme of the `HorizBar`.
    #[inline]
    pub const fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

        self
    }
}

impl<R: Render> Draw<R> for HorizBar {
    fn draw(&self, buf: &mut R, area: Area)
    {
        if area.is_collapsed() {
            return;
        }

        let top_left = area.top_left();
        buf.hfill(top_left, self.theme.body, area.width as usize);
        buf.putc_abs(top_left, self.theme.beg);
        buf.putc_abs(area.top_right().sub_x(1), self.theme.end);
    }
}

impl Proportional for HorizBar {
    fn proportions(&self) -> Proportions
    {
        use crate::layout::Range;

        Proportions {
            width: Range::flexible(),
            height: Range::fixed(1),
        }
    }
}

/// Draws a vertical bar starting at the top-left corner of the render area and
/// spanning the full height of the render area.
#[derive(Debug, Clone, Default)]
pub struct VertBar {
    pub theme: Theme,
}

impl VertBar {
    /// Creates a new `VertBar`.
    #[inline]
    pub const fn new() -> Self
    {
        Self {
            theme: Theme::default(),
        }
    }

    #[inline]
    pub const fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

        self
    }
}

impl<R: Render> Draw<R> for VertBar {
    fn draw(&self, buf: &mut R, area: Area)
    {
        if area.is_collapsed() {
            return;
        }

        let top_left = area.top_left();
        buf.vfill(top_left, self.theme.body, area.height as usize);
        buf.putc_abs(top_left, self.theme.beg);
        buf.putc_abs(area.bottom_left().sub_y(1), self.theme.end);
    }
}

impl Proportional for VertBar {
    fn proportions(&self) -> Proportions
    {
        use crate::layout::Range;

        Proportions {
            width: Range::fixed(1),
            height: Range::flexible(),
        }
    }
}
