use crate::style::{StyledStr, StyledChar, WithStyle};
use crate::layout::{Area, Pos, Justify};

// FIXME: all methods should take a limiting area.

/// Painting rendered widgets.
///
/// Types can implement this trait to signify that widgets can be painted onto
/// them.
pub trait Paint {
    fn area(&self) -> Area;

    /// Print in an area.
    ///
    /// The position is relative and `text` is truncated to fit `area`.
    fn print<'s, S>(&mut self, pos: Pos, text: S, area: Area)
    where
        S: Into<StyledStr<'s>>;

    fn putc<T>(&mut self, pos: Pos, c: T)
    where
        T: Into<StyledChar>;

    fn hfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>;

    fn vfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>;

    fn clear(&mut self);

    fn show_cursor(&mut self);

    fn hide_cursor(&mut self);

    fn move_cursor(&mut self, pos: Pos);

    // Helper methods.

    /// Print absolute.
    ///
    /// Same as `print` but the area is the painter's full area.
    fn print_abs<'s, S>(&mut self, pos: Pos, text: S)
    where
        S: Into<StyledStr<'s>>
    {
        self.print(pos, text, self.area());
    }

    /// Print justified in an area.
    fn printj<'s, S>(&mut self, text: S, j: Justify, area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_void() {
            return;
        }

        let text: StyledStr = text.into();
        // TODO: utf8 support.
        let text_width = text.content.len();
        // TODO: implement direct slicing of `StyledStr`.
        let text
            = text.content[..std::cmp::min(text_width, area.width as usize)]
                .with_style(|_| text.style);

        let pos = match j {
            Justify::Left(y) => Pos {
                x: 0,
                y
            },
            Justify::HCentre(y) => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y,
            },
            Justify::Right(y) => Pos {
                x: (area.width as usize - text_width) as u16,
                y,
            },
            Justify::Top(x) => Pos {
                x,
                y: 0,
            },
            Justify::VCentre(x) => Pos {
                x,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Bottom(x) => Pos {
                x,
                y: area.height.saturating_sub(1),
            },
            Justify::TopLeft => Pos::ZERO,
            Justify::TopCentre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: 0,
            },
            Justify::TopRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: 0,
            },
            Justify::CentreLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Centre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::CentreRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::BottomLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomCentre => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1),
            },
        };

        self.print(pos, text, area);
    }
}
