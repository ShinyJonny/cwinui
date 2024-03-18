use super::Widget;
use crate::{Pos, Area};
use crate::layout::{Layout, Proportions, Alignment};
use crate::paint::Paint;


/// Renders the wrapped widget in the largest area possible.
#[derive(Debug)]
pub struct Max<T: Widget + Layout>(pub T);

impl<T: Widget + Layout> Widget for Max<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
            // TODO: error handling of insufficient dimensions.
            .unwrap_or_else(|d| d);

        self.0.render(buf, Area::from_parts(area.top_left(), dim));
    }
}

impl<T: Widget + Layout> Layout for Max<T> {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().max()
    }
}


/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug)]
pub struct Min<T: Widget + Layout>(pub T);

impl<T: Widget + Layout> Widget for Min<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
            // TODO: error handling of insufficient dimensions.
            .unwrap_or_else(|d| d);

        self.0.render(buf, Area::from_parts(area.top_left(), dim));
    }
}

impl<T: Widget + Layout> Layout for Min<T> {
    #[inline]
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
    align_method!(top_center, TopCenter);
    align_method!(top_right, TopRight);
    align_method!(center_left, CenterLeft);
    align_method!(center, Center);
    align_method!(center_right, CenterRight);
    align_method!(bottom_left, BottomLeft);
    align_method!(bottom_center, BottomCenter);
    align_method!(bottom_right, BottomRight);
}

impl<T: Widget + Layout> Widget for Align<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
            // TODO: error handling of insufficient dimensions.
            .unwrap_or_else(|d| d);

        let pos = match self.alignment {
            Alignment::TopLeft => area.top_left(),
            Alignment::TopCenter => area.top_left()
                .add_x((area.width - dim.width) / 2),
            Alignment::TopRight => area.top_right()
                .sub_x(dim.width),
            Alignment::CenterLeft => area.top_left()
                .add_y((area.height - dim.height) / 2),
            Alignment::Center => area.top_left()
                .add_x((area.width - dim.width) / 2)
                .add_y((area.height - dim.height) / 2),
            Alignment::CenterRight => area.top_right()
                .sub_x(dim.width)
                .add_y((area.height - dim.height) / 2),
            Alignment::BottomLeft => area.bottom_left()
                .sub_y(dim.height),
            Alignment::BottomCenter => area.bottom_left()
                .add_x((area.width - dim.width) / 2)
                .sub_y(dim.height),
            Alignment::BottomRight => area.bottom_right()
                .sub_x(dim.width)
                .sub_y(dim.height),
        };

        self.inner.render(buf, Area::from_parts(pos, dim));
    }
}

impl<T: Widget + Layout> Layout for Align<T> {
    #[inline]
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
                let dim = area.dimensions()
                    .satisfy(self.0.proportions())
                    // TODO: error handling of insufficient dimensions.
                    .unwrap_or_else(|d| d);

                let inner_area = Area::from_parts(Pos::ZERO, dim)
                    .align_to(area, Alignment::$al);

                self.0.render(buf, inner_area);
            }
        }

        impl<T: Widget + Layout> Layout for $al<T> {
            #[inline]
            fn proportions(&self) -> Proportions
            {
                self.0.proportions().expand()
            }
        }
    }
}


def_static_align!(TopLeft);
def_static_align!(TopCenter);
def_static_align!(TopRight);
def_static_align!(CenterLeft);
def_static_align!(Center);
def_static_align!(CenterRight);
def_static_align!(BottomLeft);
def_static_align!(BottomCenter);
def_static_align!(BottomRight);
