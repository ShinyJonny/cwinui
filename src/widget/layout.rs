use super::Widget;
use crate::{Pos, Area};
use crate::layout::{Proportional, Proportions, Alignment};
use crate::paint::Paint;


/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug)]
pub struct Min<T: Widget + Proportional>(pub T);

impl<T: Widget + Proportional> Widget for Min<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
            .unwrap_or_else(|d| d);

        self.0.render(buf, Area::from_parts(area.top_left(), dim));
    }
}

impl<T: Widget + Proportional> Proportional for Min<T> {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().collapse()
    }
}


/// Align the contained widget dynamically, based on `alignment`.
#[derive(Debug)]
pub struct Align<T: Widget + Proportional> {
    pub inner:     T,
    pub alignment: Alignment,
}

macro_rules! align_method {
    ($name:ident, $al:ident) => {
        pub const fn $name(inner: T) -> Self
        {
            Self {
                inner,
                alignment: Alignment::$al,
            }
        }
    }
}

impl<T: Widget + Proportional> Align<T> {
    align_method!(top_left,      TopLeft);
    align_method!(top_center,    TopCenter);
    align_method!(top_right,     TopRight);
    align_method!(center_left,   CenterLeft);
    align_method!(center,        Center);
    align_method!(center_right,  CenterRight);
    align_method!(bottom_left,   BottomLeft);
    align_method!(bottom_center, BottomCenter);
    align_method!(bottom_right,  BottomRight);
}

impl<T: Widget + Proportional> Widget for Align<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
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

impl<T: Widget + Proportional> Proportional for Align<T> {
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
        pub struct $al<T: Widget + Proportional>(pub T);

        impl<T: Widget + Proportional> Widget for $al<T> {
            fn render(&self, buf: &mut impl Paint, area: Area)
            {
                let dim = area.dimensions()
                    .satisfy(self.0.proportions())
                    .unwrap_or_else(|d| d);

                let inner_area = Area::from_parts(Pos::ZERO, dim)
                    .align_to(area, Alignment::$al);

                self.0.render(buf, inner_area);
            }
        }

        impl<T: Widget + Proportional> Proportional for $al<T> {
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


/// Adds padding to the contained widget.
pub struct Pad<T: Widget> {
    pub inner:  T,
    pub top:    u16,
    pub right:  u16,
    pub bottom: u16,
    pub left:   u16,
}

impl<T: Widget> Pad<T> {
    pub const fn new(inner: T) -> Self
    {
        Self {
            inner,
            top:    0,
            right:  0,
            bottom: 0,
            left:   0,
        }
    }

    pub const fn top(mut self,    val: u16) -> Self { self.top    = val; self }
    pub const fn right(mut self,  val: u16) -> Self { self.right  = val; self }
    pub const fn bottom(mut self, val: u16) -> Self { self.bottom = val; self }
    pub const fn left(mut self,   val: u16) -> Self { self.left   = val; self }
}

impl<T: Widget> Widget for Pad<T> {
    #[inline]
    fn render(&self, buf: &mut impl Paint, area: Area)
    {
        let horiz_pad = self.left + self.right;
        let vert_pad  = self.top  + self.bottom;

        if area.width.saturating_sub(horiz_pad) == 0
            || area.height.saturating_sub(vert_pad) == 0
        {
            return;
        }

        let area = Area {
            x:      area.x + self.left,
            y:      area.y + self.top,
            width:  area.width  - horiz_pad,
            height: area.height - vert_pad,
        };

        self.inner.render(buf, area);
    }
}

impl<T: Widget> Proportional for Pad<T>
where
    T: Proportional
{
    fn proportions(&self) -> Proportions {
        use crate::layout::P;

        #[inline]
        pub fn pad_p(initial: P, pad: u16) -> P
        {
            match initial {
                P::Flexible        => P::From(pad),
                P::Fixed(v)        => P::Fixed(v + pad),
                P::To(max)         => P::Range(pad, max + pad),
                P::From(min)       => P::From(min + pad),
                P::Range(min, max) => P::Range(min + pad, max + pad),
            }
        }

        let inner_prop = self.inner.proportions();

        Proportions {
            horiz: pad_p(inner_prop.horiz, self.left + self.right),
            vert:  pad_p(inner_prop.vert,  self.top  + self.bottom),
        }
    }
}
