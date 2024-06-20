use crate::Area;
use crate::style::StyledChar;
use crate::layout::{Proportional, Proportions};

use super::{Draw, Render};


/// Fills the space with a [`StyledChar`].
#[derive(Debug, Clone)]
pub struct Filler(pub StyledChar);

impl<R: Render> Draw<R> for Filler {
    fn draw(&self, buf: &mut R, area: Area)
    {
        buf.fill(self.0, area);
    }
}

impl Proportional for Filler {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}
