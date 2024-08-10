use crate::layout::{Proportional, Proportions};
use crate::style::{Style, WithStyle};
use crate::util::offset;
use crate::{Dim, Draw, Area, Pos};
use crate::buffer::{Buffer, Cursor};
use crate::render::Render;


/// A buffered canvas that allows widgets to draw onto it.
#[derive(Clone)]
pub struct Canvas {
    width: u16,
    height: u16,
    chars: Vec<char>,
    styles: Vec<Style>,
    cursor: Cursor,
}

impl Canvas {
    /// Allocates a new `Canvas` with the size of `dimensions`.
    pub fn new(dimensions: Dim) -> Self
    {
        let size = dimensions.width as usize * dimensions.height as usize;

        Self {
            width: dimensions.width,
            height: dimensions.height,
            chars: vec![' '; size],
            styles: vec![Style::default().clean(); size],
            cursor: Cursor { x: 0, y: 0, hidden: true },
        }
    }

    /// Exposes the `Render` interface.
    #[inline]
    pub fn renderer(&mut self) -> impl Render + '_
    {
        Buffer::new(self.width, self.height, &mut self.chars, &mut self.styles, &mut self.cursor)
    }
}

impl std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.debug_struct("Canvas")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl<R: Render> Draw<R> for Canvas {
    fn draw(&self, buf: &mut R, area: Area)
    {
        let width = std::cmp::min(area.width, self.width);
        let height = std::cmp::min(area.height, self.height);

        // FIXME: very inefficient due to bounds checking, needs to be done via
        // diffing or some other method on `Render` instead.
        // Also, having separate style and char bufs seems inefficient here.
        for y in 0..height {
            for x in 0..width {
                let offset = offset!(x, y, self.width);
                let c = self.chars[offset]
                    .with_style(|_| self.styles[offset]);
                buf.set_char(Pos { x: x + area.x, y: y + area.y }, c);
            }
        }

        // NOTE: we ignore cursors.
    }
}

impl Proportional for Canvas {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        Proportions::fixed(Dim {
            width: self.width,
            height: self.height,
        })
    }
}
