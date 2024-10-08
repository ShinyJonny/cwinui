use crate::layout::{Area, Pos, Dim, Justify};
use crate::style::{AsStyledStr, StyledChar};

/// Render - the basic mechanism for drawing widgets.
///
/// Types can implement this trait in order to render [`Draw`]ables.
pub trait Render {
    /// Get the paint area.
    fn area(&self) -> Area;

    /// Set a `StyledStr`.
    ///
    /// # Panics
    ///
    /// When out of bounds.
    fn set_str<S: AsStyledStr>(&mut self, pos: Pos, text: S);

    /// Set a `StyledChar`.
    ///
    /// # Panics
    ///
    /// When out of bounds.
    fn set_char<C>(&mut self, pos: Pos, c: C)
    where
        C: Into<StyledChar>;

    /// Clear the buffer.
    fn clear(&mut self);

    /// Show the cursor.
    fn show_cursor(&mut self);

    /// Hide the cursor.
    fn hide_cursor(&mut self);

    /// Move the cursor.
    fn move_cursor(&mut self, pos: Pos);

    // Helper methods.

    /// Get the dimensions of the paint area.
    #[inline]
    fn dimensions(&self) -> Dim
    {
        self.area().dimensions()
    }

    /// Fill an area with `c`.
    #[inline]
    fn fill<T>(&mut self, c: T, area: Area)
    where
        T: Into<StyledChar>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);
        if area.is_collapsed() {
            return;
        }

        let c = c.into();

        for y in 0..area.height {
            for x in 0..area.width {
                let x = area.x + x;
                let y = area.y + y;
                self.set_char(Pos { x, y }, c);
            }
        }
    }

    /// Fill a horizontal line with `c`,  of length `len` and starting a `pos`.
    #[inline]
    fn hfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let dim = self.dimensions();

        if pos.x >= dim.width || pos.y >= dim.height {
            return;
        }

        let fill_len = std::cmp::min((dim.width - pos.x) as usize, len);
        let c = c.into();

        for i in 0..fill_len {
            self.set_char(pos.add_x(i as u16), c);
        }
    }

    /// Fill a vertical line with `c`,  of length `len` and starting a `pos`.
    #[inline]
    fn vfill<T>(&mut self, pos: Pos, c: T, len: usize)
    where
        T: Into<StyledChar>
    {
        let dim = self.dimensions();

        if pos.x >= dim.width || pos.y >= dim.height {
            return;
        }

        let fill_len = std::cmp::min((dim.height - pos.y) as usize, len);
        let c = c.into();

        for i in 0..fill_len {
            self.set_char(pos.add_y(i as u16), c);
        }
    }

    /// Bounds-checked absolute printing.
    #[inline]
    fn print_abs<S: AsStyledStr>(&mut self, pos: Pos, text: S)
    {
        let area = self.area();

        if pos.x >= area.width || pos.y >= area.height {
            return;
        }

        let text = text.as_styled_str();

        // TODO: utf8 support.
        let print_width = std::cmp::min(
            text.content.len(),
            area.width as usize - pos.x as usize
        );

        self.set_str(pos, text.slice(..print_width));
    }

    /// Bounds-checked absolute printing of a styled character.
    #[inline]
    fn putc_abs<T>(&mut self, pos: Pos, c: T)
    where
        T: Into<StyledChar>
    {
        let area = self.area();

        if pos.x >= area.width || pos.y >= area.height {
            return;
        }

        self.set_char(pos, c);
    }

    /// Bounds-checked print, relative to `area`.
    #[inline]
    fn print<S: AsStyledStr>(&mut self, pos: Pos, text: S, area: Area)
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        let abs_x = area.x + pos.x;
        let abs_y = area.y + pos.y;

        if abs_x >= area.x + area.width || abs_y >= area.y + area.height {
            return;
        }

        let text = text.as_styled_str();
        let right_max  = area.x as usize + area.width as usize;

        // TODO: utf8 support.
        let print_width = std::cmp::min(
            text.content.len(),
            right_max - abs_x as usize
        );

        self.set_str(Pos{x:abs_x,y:abs_y}, text.slice(..print_width));
    }

    /// Bounds-checked print of a styled character, relative to `area`.
    #[inline]
    fn putc<T>(&mut self, pos: Pos, c: T, area: Area)
    where
        T: Into<StyledChar>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if pos.x >= area.width || pos.y >= area.height {
            return;
        }

        self.set_char(pos + area.top_left(), c);
    }

    /// Print justified in an area.
    #[inline]
    fn jprint<S: AsStyledStr>(&mut self, text: S, j: Justify, area: Area)
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_collapsed() {
            return;
        }

        let text = text.as_styled_str();
        // TODO: utf8 support.
        let text_width = std::cmp::min(text.content.len(), area.width as usize);

        let pos = match j {
            Justify::Left(y) => Pos {
                x: 0,
                y
            },
            Justify::HCenter(y) => Pos {
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
            Justify::VCenter(x) => Pos {
                x,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Bottom(x) => Pos {
                x,
                y: area.height.saturating_sub(1),
            },
            Justify::TopLeft => Pos::ZERO,
            Justify::TopCenter => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: 0,
            },
            Justify::TopRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: 0,
            },
            Justify::CenterLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Center => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::CenterRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::BottomLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomCenter => Pos {
                x: (area.width as usize - text_width) as u16 / 2,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomRight => Pos {
                x: (area.width as usize - text_width) as u16,
                y: area.height.saturating_sub(1),
            },
        };

        self.print(pos, text.slice(..text_width), area);
    }

    /// Putc justified in an area.
    #[inline]
    fn jputc<C>(&mut self, c: C, j: Justify, area: Area)
    where
        C: Into<StyledChar>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_collapsed() {
            return;
        }

        // TODO: utf8 support.

        let pos = match j {
            Justify::Left(y) => Pos {
                x: 0,
                y
            },
            Justify::HCenter(y) => Pos {
                x: area.width.saturating_sub(1) / 2,
                y,
            },
            Justify::Right(y) => Pos {
                x: area.width.saturating_sub(1),
                y,
            },
            Justify::Top(x) => Pos {
                x,
                y: 0,
            },
            Justify::VCenter(x) => Pos {
                x,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Bottom(x) => Pos {
                x,
                y: area.height.saturating_sub(1),
            },
            Justify::TopLeft => Pos::ZERO,
            Justify::TopCenter => Pos {
                x: area.width.saturating_sub(1) / 2,
                y: 0,
            },
            Justify::TopRight => Pos {
                x: area.width.saturating_sub(1),
                y: 0,
            },
            Justify::CenterLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::Center => Pos {
                x: area.width.saturating_sub(1) / 2,
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::CenterRight => Pos {
                x: area.width.saturating_sub(1),
                y: area.height.saturating_sub(1) / 2,
            },
            Justify::BottomLeft => Pos {
                x: 0,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomCenter => Pos {
                x: area.width.saturating_sub(1) / 2,
                y: area.height.saturating_sub(1),
            },
            Justify::BottomRight => Pos {
                x: area.width.saturating_sub(1),
                y: area.height.saturating_sub(1),
            },
        };

        self.putc(pos, c, area);
    }
}


/// The type can be drawn with a [`Render`]er.
pub trait Draw<R: Render> {
    /// Draws the widget onto `buf`.
    fn draw(&self, buf: &mut R, area: Area);
}

impl<T, R: Render> Draw<R> for &T
where
    T: Draw<R>,
{
    fn draw(&self, buf: &mut R, area: Area)
    {
        T::draw(*self, buf, area);
    }
}

impl<T, R: Render> Draw<R> for &mut T
where
    T: Draw<R>,
{
    fn draw(&self, buf: &mut R, area: Area)
    {
        T::draw(*self, buf, area);
    }
}
