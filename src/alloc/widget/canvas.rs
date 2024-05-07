use crate::layout::{Proportional, Proportions};
use crate::style::WithStyle;
use crate::util::offset;
use crate::{Dim, Draw, Area, Pos};
use crate::alloc::buffer::Buffer;
use crate::widget::Paint;


/// A buffered canvas that allows widgets to draw onto it.
#[derive(Clone)]
pub struct Canvas {
    buffer: Buffer,
}

impl Canvas {
    /// Allocates a new `Canvas` with the size of `dimensions`.
    pub fn new(dimensions: Dim) -> Self
    {
        Self {
            buffer: Buffer::new(dimensions.width, dimensions.height),
        }
    }

    /// Exposes the `Paint` interface.
    #[inline]
    pub fn painter(&mut self) -> &mut impl Paint
    {
        &mut self.buffer
    }
}

impl std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.debug_struct("Canvas")
            .field("width", &self.buffer.width)
            .field("height", &self.buffer.height)
            .finish()
    }
}

impl<P: Paint> Draw<P> for Canvas {
    fn draw(&self, buf: &mut P, area: Area)
    {
        let width = std::cmp::min(area.width, self.buffer.width);
        let height = std::cmp::min(area.height, self.buffer.height);

        // FIXME: very inefficient due to bounds checking, needs to be done via
        // diffing or some other method on `Paint` instead.
        // Also, having separate style and char bufs seems inefficient here.
        for y in 0..height {
            for x in 0..width {
                let offset = offset!(x as usize, y as usize,
                    self.buffer.width as usize);
                let c = self.buffer.chars[offset]
                    .with_style(|_| self.buffer.styles[offset]);
                buf.paint_char(Pos { x: x + area.x, y: y + area.y }, c);
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
            width: self.buffer.width,
            height: self.buffer.height,
        })
    }
}
