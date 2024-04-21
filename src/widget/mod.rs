use termion::event::Event;

use crate::{Area, Dim, Pos};
use crate::style::{StyledChar, StyledStr};
use crate::layout::Justify;


pub mod bar;
pub mod inputline;
pub mod menu;
pub mod prompt;
pub mod frame;
pub mod canvas;
pub mod layout;
pub mod flex;
pub mod split;

pub use split::{Row, Col};
pub use flex::{FlexCol, FlexRow};
pub use bar::{HorizBar, VertBar};
pub use inputline::InputLine;
pub use menu::Menu;
pub use prompt::Prompt;
pub use frame::Frame;
pub use canvas::Canvas;


/// Painting rendered widgets.
///
/// Types can implement this trait to allow widgets to be painted onto them.
pub trait Paint {
    /// Get the render area.
    fn area(&self) -> Area;

    /// Paint a `StyledStr`.
    ///
    /// # Panics
    ///
    /// When out of bounds.
    fn paint_str<'s, S>(&mut self, pos: Pos, text: S)
    where
        S: Into<StyledStr<'s>>;

    /// Paint a `StyledChar`.
    ///
    /// # Panics
    ///
    /// When out of bounds.
    fn paint_char<C>(&mut self, pos: Pos, c: C)
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

    /// Get the dimensions of the render area.
    #[inline]
    fn dimensions(&self) -> Dim
    {
        self.area().dimensions()
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
            self.paint_char(pos.add_x(i as u16), c);
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
            self.paint_char(pos.add_y(i as u16), c);
        }
    }

    /// Bounds-checked absolute printing.
    #[inline]
    fn print_abs<'s, S>(&mut self, pos: Pos, text: S)
    where
        S: Into<StyledStr<'s>>
    {
        let area = self.area();

        if pos.x >= area.width || pos.y >= area.height {
            return;
        }

        let text: StyledStr<'_> = text.into();

        // TODO: utf8 support.
        let print_width = std::cmp::min(
            text.content.len(),
            area.width as usize - pos.x as usize
        );

        self.paint_str(pos, text.slice(..print_width));
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

        self.paint_char(pos, c);
    }

    /// Bounds-checked print, relative to `area`.
    #[inline]
    fn print<'s, S>(&mut self, pos: Pos, text: S, area: Area)
    where
        S: Into<StyledStr<'s>>
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

        let text: StyledStr<'_> = text.into();
        let right_max  = area.x as usize + area.width as usize;

        // TODO: utf8 support.
        let print_width = std::cmp::min(
            text.content.len(),
            right_max - abs_x as usize
        );

        self.paint_str(Pos{x:abs_x,y:abs_y}, text.slice(..print_width));
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

        self.paint_char(pos + area.top_left(), c);
    }

    /// Print justified in an area.
    #[inline]
    fn printj<'s, S>(&mut self, text: S, j: Justify, area: Area)
    where
        S: Into<StyledStr<'s>>
    {
        if !self.area().overlaps(area) {
            return;
        }
        let area = self.area().intersection(area);

        if area.is_collapsed() {
            return;
        }

        let text: StyledStr = text.into();
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
}


/// Structures that can be rendered as widgets.
pub trait Widget<P: Paint> {
    /// Renders the widget onto `buf`.
    fn render(&self, buf: &mut P, area: Area);
}

impl<T, P: Paint> Widget<P> for &T
where
    T: Widget<P>,
{
    fn render(&self, buf: &mut P, area: Area)
    {
        T::render(self, buf, area);
    }
}


/// Interactive widgets that can process events.
pub trait InteractiveWidget {
    /// Processes an event.
    fn process_event(&mut self, e: Event);
}


/// A dummy widget that does nothing.
#[derive(Debug, Clone, Copy)]
pub struct Dummy;

impl<P: Paint> Widget<P> for Dummy {
    fn render(&self, _buf: &mut P, _area: Area) {}
}

impl InteractiveWidget for Dummy {
    fn process_event(&mut self, _e: Event) {}
}

impl crate::layout::Proportional for Dummy {
    fn proportions(&self) -> crate::layout::Proportions
    {
        crate::layout::Proportions::flexible()
    }
}
