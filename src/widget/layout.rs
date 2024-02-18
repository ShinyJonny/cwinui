use super::Widget;
use crate::{Area, Dim};
use crate::layout::{Layout, Proportions, Alignment};
use crate::paint::Paint;


/// Renders the wrapped widget in the largest area possible.
#[derive(Debug)]
pub struct Max<T: Widget + Layout>(pub T);

impl<T: Widget + Layout> Widget for Max<T> {

    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let Dim { width, height } = area.dimensions()
            .satisfy(self.proportions())
            // TODO: error handling.
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

impl<T: Widget + Layout> Layout for Max<T> {
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().max()
    }
}


/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug)]
pub struct Min<T: Widget + Layout>(pub T);

impl<T: Widget + Layout> Widget for Min<T> {

    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let Dim { width, height } = area.dimensions()
            .satisfy(self.proportions())
            // TODO: error handling.
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

impl<T: Widget + Layout> Layout for Min<T> {
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().min()
    }
}


/// Align the contained widget dynamically, based on `alignment`.
#[derive(Debug)]
pub struct Align<T: Widget + Layout> {
    pub inner: T,
    pub alignment: Alignment,
}

macro_rules! align_method {
    ($name:ident, $al:ident) => {
        pub fn $name(inner: T) -> Self
        {
            Self {
                inner,
                alignment: Alignment::$al,
            }
        }
    }
}

impl<T: Widget + Layout> Align<T> {
    align_method!(top_left, TopLeft);
    align_method!(top_centre, TopCentre);
    align_method!(top_right, TopRight);
    align_method!(centre_left, CentreLeft);
    align_method!(centre, Centre);
    align_method!(centre_right, CentreRight);
    align_method!(bottom_left, BottomLeft);
    align_method!(bottom_centre, BottomCentre);
    align_method!(bottom_right, BottomRight);
}

impl<T: Widget + Layout> Widget for Align<T> {
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let inner_dim = area.dimensions()
            .satisfy(self.inner.proportions())
            // TODO: handle errors.
            .unwrap_or_else(|e| e);

        if inner_dim.width == 0 || inner_dim.height == 0 {
            return;
        }

        let pos = match self.alignment {
            Alignment::TopLeft => area.top_left(),
            Alignment::TopCentre => area.top_left()
                .add_x((area.width - inner_dim.width) / 2),
            Alignment::TopRight => area.top_right()
                .sub_x(inner_dim.width),
            Alignment::CentreLeft => area.top_left()
                .add_y((area.height - inner_dim.height) / 2),
            Alignment::Centre => area.top_left()
                .add_x((area.width - inner_dim.width) / 2)
                .add_y((area.height - inner_dim.height) / 2),
            Alignment::CentreRight => area.top_right()
                .sub_x(inner_dim.width)
                .add_y((area.height - inner_dim.height) / 2),
            Alignment::BottomLeft => area.bottom_left()
                .sub_y(inner_dim.height),
            Alignment::BottomCentre => area.bottom_left()
                .add_x((area.width - inner_dim.width) / 2)
                .sub_y(inner_dim.height),
            Alignment::BottomRight => area.bottom_right()
                .sub_x(inner_dim.width)
                .sub_y(inner_dim.height),
        };

        self.inner.render(buf, Area::from_parts(pos, inner_dim));
    }
}

impl<T: Widget + Layout> Layout for Align<T> {
    fn proportions(&self) -> Proportions
    {
        self.inner.proportions().expand()
    }
}


macro_rules! def_static_align {
    ($al:ident) => {
        #[doc = "Align the contained widget to"]
        #[doc = concat!("[`Alignment::", stringify!($al), "`]")]
        #[doc = "."]
        pub struct $al<T: Widget + Layout>(pub T);

        impl<T: Widget + Layout> Widget for $al<T> {
            fn render(&self, buf: &mut impl Paint, area: Area)
            {
                let inner_dim = area.dimensions()
                    .satisfy(self.0.proportions())
                    // TODO: handle errors.
                    .unwrap_or_else(|e| e);

                if inner_dim.width == 0 || inner_dim.height == 0 {
                    return;
                }

                let inner_area = Area::from_parts(crate::layout::Pos::ZERO, inner_dim)
                    .align_to(area, Alignment::$al);
                self.0.render(buf, inner_area);
            }
        }
    }
}


def_static_align!(TopLeft);
def_static_align!(TopCentre);
def_static_align!(TopRight);
def_static_align!(CentreLeft);
def_static_align!(Centre);
def_static_align!(CentreRight);
def_static_align!(BottomLeft);
def_static_align!(BottomCentre);
def_static_align!(BottomRight);
