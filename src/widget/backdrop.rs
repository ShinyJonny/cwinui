use crate::layout::{Proportional, Proportions};

use super::{Render, Draw};


/// Allows two widgets to be drawn in the same area, on top of each other.
///
/// First, `B` is drawn and then `F` is drawn in the same area.
#[derive(Debug, Clone)]
pub struct Backdrop<F, B> {
    pub fg: F,
    pub bg: B,
}

impl<F: Draw<R>, B: Draw<R>, R: Render> Draw<R> for Backdrop<F, B> {
    fn draw(&self, buf: &mut R, area: crate::Area)
    {
        self.bg.draw(buf, area);
        self.fg.draw(buf, area);
    }
}

impl<F, B> Proportional for Backdrop<F, B>
where
    F: Proportional,
    B: Proportional,
{
    fn proportions(&self) -> Proportions
    {
        self.fg.proportions()
            .join(self.bg.proportions())
    }
}
