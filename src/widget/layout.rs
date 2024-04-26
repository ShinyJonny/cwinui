use super::Widget;
use crate::{Area, Pos};
use crate::layout::{Alignment, Proportional, Proportions, Range};
use crate::widget::Paint;


/// Renders the wrapped widget in the smallest area possible.
#[derive(Debug, Clone)]
pub struct Min<T: Proportional>(pub T);

impl<T: Widget<P> + Proportional, P: Paint> Widget<P> for Min<T> {
    #[inline]
    fn render(&self, buf: &mut P, area: Area)
    {
        let dim = self.proportions()
            .fit_into(area.dimensions())
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


/// Align the contained widget based on `alignment`.
#[derive(Debug, Clone)]
pub struct Align<T: Proportional> {
    pub inner:     T,
    pub alignment: Alignment,
}

macro_rules! align_method {
    ($name:ident, $al:ident) => {
        #[doc = "Align to"]
        #[doc = concat!("[`Alignment::", stringify!($al), "`]")]
        #[doc = "."]
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
        let dim = self.inner.proportions()
            .fit_into(area.dimensions())
            .unwrap_or_else(|d| d);

        let inner_area = Area::from_parts(Pos::ZERO, dim)
            .align_to(area, self.alignment);

        self.inner.render(buf, inner_area);
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
        #[derive(Debug, Clone)]
        pub struct $al<T: Proportional>(pub T);

        impl<T: Widget<P> + Proportional, P: Paint> Widget<P> for $al<T> {
            fn render(&self, buf: &mut P, area: Area)
            {
                let dim = self.0.proportions()
                    .fit_into(area.dimensions())
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
#[derive(Debug, Clone)]
pub struct Pad<T> {
    pub inner:  T,
    pub top:    u16,
    pub right:  u16,
    pub bottom: u16,
    pub left:   u16,
}

impl<T> Pad<T> {
    /// Creates `Pad<T>` with the default padding (zero for all sides).
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

    /// Adjusts the *top* padding.
    pub const fn top(mut self,    val: u16) -> Self { self.top    = val; self }
    /// Adjusts the *right* padding.
    pub const fn right(mut self,  val: u16) -> Self { self.right  = val; self }
    /// Adjusts the *bottom* padding.
    pub const fn bottom(mut self, val: u16) -> Self { self.bottom = val; self }
    /// Adjusts the *left* padding.
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


/// Container with its own proportions.
#[derive(Debug, Clone)]
pub struct Container<T> {
    pub inner: T,
    pub proportions: Proportions,
}

impl Container<super::Void> {
    /// Creates a `Container` containing nothing.
    #[inline]
    pub const fn empty() -> Self
    {
        Self {
            inner: super::Void,
            proportions: Proportions::flexible(),
        }
    }
}

impl<T> Container<T> {
    /// Wraps `inner` in a `Container` with default proportions.
    #[inline]
    pub const fn new(inner: T) -> Self
    {
        Self {
            inner,
            proportions: Proportions::flexible(),
        }
    }

    /// Adjusts the proportions of the `Container`.
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


/// In case if insufficient dimensions, renders `F` instead of `T`.
#[derive(Debug, Clone)]
pub struct Fallback<T: Proportional, F> {
    pub inner: T,
    pub fallback: F,
}

impl<T: Proportional, F, P: Paint> Widget<P> for Fallback<T, F>
where
    T: Widget<P>,
    F: Widget<P>,
{
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.dimensions().satisfies(self.inner.proportions()) {
            self.inner.render(buf, area);
        } else {
            self.fallback.render(buf, area);
        }
    }
}

impl<T: Proportional, F> Proportional for Fallback<T, F> {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        self.inner.proportions()
    }
}
