use super::Widget;
use crate::{Area, Dim};
use crate::layout::{Proportional, Proportions, Alignment};
use crate::paint::Paint;

/// Renders the wrapped widget in the largest area possible.
#[derive(Debug)]
pub struct Max<T: Widget + Proportional>(pub T);

impl<T: Widget + Proportional> Widget for Max<T> {

    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let Dim { width, height } = area.dimensions()
            .satisfy(self.proportions())
            .unwrap_or_else(|e| e);

        let inner_area = Area {
            x: area.x,
            y: area.y,
            width,
            height,
        };

        self.0.render(buf, inner_area);
    }
}

impl<T: Widget + Proportional> Proportional for Max<T> {
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().max()
    }
}

/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug)]
pub struct Min<T: Widget + Proportional>(pub T);

impl<T: Widget + Proportional> Widget for Min<T> {

    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let Dim { width, height } = area.dimensions()
            .satisfy(self.proportions())
            .unwrap_or_else(|e| e);

        let inner_area = Area {
            x: area.x,
            y: area.y,
            width,
            height,
        };

        self.0.render(buf, inner_area);
    }
}

impl<T: Widget + Proportional> Proportional for Min<T> {
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().min()
    }
}

#[derive(Debug)]
pub struct Align<T: Widget + Proportional> {
    inner: T,
    alignment: Alignment,
}

impl<T: Widget + Proportional> Align<T> {
    pub fn new(inner: T, alignment: Alignment) -> Self
    {
        Self {
            inner,
            alignment,
        }
    }
}

impl<T: Widget + Proportional> Widget for Align<T> {
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions().satisfy(self.inner.proportions())
            .unwrap_or_else(|e| e);

        if dim.width == 0 || dim.height == 0 {
            return;
        }

        let pos = match self.alignment {
            Alignment::TopLeft      => area.top_left(),
            Alignment::TopCentre    => area.top_left()
                .add_x((area.width - dim.width) / 2),
            Alignment::TopRight     => area.top_right()
                .sub_x(dim.width),
            Alignment::CentreLeft   => area.top_left()
                .add_y((area.height - dim.height) / 2),
            Alignment::Centre       => area.top_right()
                .add_x((area.width - dim.width) / 2)
                .add_y((area.height - dim.height) / 2),
            Alignment::CentreRight  => area.top_right()
                .sub_x(dim.width)
                .add_y((area.height - dim.height) / 2),
            Alignment::BottomLeft   => area.bottom_left()
                .sub_y(dim.height),
            Alignment::BottomCentre => area.bottom_left()
                .add_x((area.width - dim.width) / 2)
                .sub_y(dim.height),
            Alignment::BottomRight  => area.bottom_right()
                .sub_x(dim.width)
                .sub_y(dim.height),
        };

        self.inner.render(buf, Area::from_parts(pos, dim));
    }
}

impl<T: Widget + Proportional> Proportional for Align<T> {
    fn proportions(&self) -> Proportions
    {
        self.inner.proportions().expand()
    }
}
