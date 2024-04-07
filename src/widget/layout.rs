use super::Widget;
use crate::{Area, Pos};
use crate::layout::{Alignment, Proportional, Proportions, Range};
use crate::paint::Paint;


/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug)]
pub struct Min<T: Proportional>(pub T);

impl<T: Widget<P> + Proportional, P: Paint> Widget<P> for Min<T> {
    #[inline]
    fn render(&self, buf: &mut P, area: Area)
    {
        let dim = area.dimensions()
            .satisfy(self.proportions())
            .unwrap_or_else(|d| d);

        self.0.render(buf, Area::from_parts(area.top_left(), dim));
    }
}

impl<T: Proportional> Proportional for Min<T> {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        self.0.proportions().collapse()
    }
}


/// Align the contained widget dynamically, based on `alignment`.
#[derive(Debug)]
pub struct Align<T: Proportional> {
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

impl<T: Proportional> Align<T> {
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

impl<T: Widget<P> + Proportional, P: Paint> Widget<P> for Align<T> {
    #[inline]
    fn render(&self, buf: &mut P, area: Area)
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

impl<T: Proportional> Proportional for Align<T> {
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
        #[derive(Debug)]
        pub struct $al<T: Proportional>(pub T);

        impl<T: Widget<P> + Proportional, P: Paint> Widget<P> for $al<T> {
            fn render(&self, buf: &mut P, area: Area)
            {
                let dim = area.dimensions()
                    .satisfy(self.0.proportions())
                    .unwrap_or_else(|d| d);

                let inner_area = Area::from_parts(Pos::ZERO, dim)
                    .align_to(area, Alignment::$al);

                self.0.render(buf, inner_area);
            }
        }

        impl<T: Proportional> Proportional for $al<T> {
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
pub struct Pad<T> {
    pub inner:  T,
    pub top:    u16,
    pub right:  u16,
    pub bottom: u16,
    pub left:   u16,
}

impl<T> Pad<T> {
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

impl<T: Widget<P>, P: Paint> Widget<P> for Pad<T> {
    #[inline]
    fn render(&self, buf: &mut P, area: Area)
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

impl<T> Proportional for Pad<T>
where
    T: Proportional
{
    fn proportions(&self) -> Proportions {
        let inner_prop = self.inner.proportions();

        Proportions {
            horiz: inner_prop.horiz.add(Range::fixed(self.left + self.right)),
            vert:  inner_prop.vert.add(Range::fixed(self.top + self.bottom)),
        }
    }
}


/// Widget that takes up the containing proportions but draws nothing.
#[derive(Debug)]
pub struct Spacer(pub Proportions);

impl<P: Paint> Widget<P> for Spacer {
    #[inline]
    fn render(&self, _buf: &mut P, _area: Area) {}
}

impl Proportional for Spacer {
    #[inline]
    fn proportions(&self) -> Proportions { self.0 }
}


/// Container that has its own proportions and simply renders the contained
/// widget.
#[derive(Debug)]
pub struct Container<T> {
    pub inner: T,
    pub proportions: Proportions,
}

impl<T> Container<T> {
    #[inline]
    pub const fn new(inner: T) -> Self
    {
        Self {
            inner,
            proportions: Proportions::flexible(),
        }
    }

    pub const fn size(mut self, proportions: Proportions) -> Self
    {
        self.proportions = proportions;

        self
    }
}

impl<T: Widget<P>, P: Paint> Widget<P> for Container<T> {
    #[inline]
    fn render(&self, buf: &mut P, area: Area)
    {
        self.inner.render(buf, area);
    }
}

impl<T> Proportional for Container<T> {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        self.proportions
    }
}
