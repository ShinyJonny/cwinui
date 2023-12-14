use crate::layout::{Proportional, Proportions};
use crate::style::WithStyle;
use crate::util::offset;
use crate::{Dim, Widget, Area, Pos};
use crate::buffer::Buffer;
use crate::paint::Paint;

pub struct Canvas {
    buffer: Buffer,
}

impl Canvas {
    pub fn new(dimensions: Dim) -> Self
    {
        Self {
            buffer: Buffer::new(dimensions.width, dimensions.height),
        }
    }

    pub fn painter(&mut self) -> &mut impl Paint
    {
        &mut self.buffer
    }
}

impl Widget for Canvas {
    fn render(&self, buf: &mut impl Paint, area: Area)
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
                let pos = Pos { x: area.x + x, y: area.y + y };
                buf.putc(pos, c);
            }
        }

        // NOTE: we ignore cursors.
    }
}

impl Proportional for Canvas {
    fn proportions(&self) -> Proportions
    {
        use crate::layout::G;

        Proportions {
            horiz: G::Fixed(self.buffer.width),
            vert: G::Fixed(self.buffer.height),
        }
    }
}
