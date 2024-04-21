use crate::layout::{Proportional, Proportions};

use super::{Paint, Widget};


/// Allows two widgets to be rendered in the same area, on top of each other.
///
/// First, `B` is rendered and then `F` is rendered in the same area.
#[derive(Debug, Clone)]
pub struct BackDrop<F, B> {
    pub fg: F,
    pub bg: B,
}

impl<F: Widget<P>, B: Widget<P>, P: Paint> Widget<P> for BackDrop<F, B> {
    fn render(&self, buf: &mut P, area: crate::Area)
    {
        self.bg.render(buf, area);
        self.fg.render(buf, area);
    }
}

impl<F, B> Proportional for BackDrop<F, B>
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
