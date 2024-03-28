/// Position coordinates.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub const ZERO: Self = Pos { x: 0, y: 0 };

    #[inline]
    pub fn saturating_add(self, rhs: Self) -> Self
    {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }

    #[inline]
    pub fn saturating_sub(self, rhs: Self) -> Self
    {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    #[inline]
    pub fn add_x(self, x: u16) -> Self
    {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }

    #[inline]
    pub fn add_y(self, y: u16) -> Self
    {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }

    #[inline]
    pub fn sub_x(self, x: u16) -> Self
    {
        Self {
            x: self.x - x,
            y: self.y,
        }
    }

    #[inline]
    pub fn sub_y(self, y: u16) -> Self
    {
        Self {
            x: self.x,
            y: self.y - y,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output
    {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output
    {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Area dimensions.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Dim {
    pub width: u16,
    pub height: u16,
}

impl Dim {
    /// Check if either of the dimensions is `0`.
    #[inline]
    pub fn is_void(&self) -> bool
    {
        self.width == 0 || self.height == 0
    }

    /// Return [`Dim`] that satisfies `proportions`.
    ///
    /// This method will always attempt to yield as large dimensions as
    /// `proportions` allow. If the dimensions can't satisfy the proportions,
    /// the `Err` value is returned with the best possible attempt at satisfying
    /// the dimensions.
    #[inline]
    pub fn satisfy(self, proportions: Proportions) -> Result<Dim, Dim>
    {
        let width  = Self::satisfy_p(self.width, proportions.horiz);
        let height = Self::satisfy_p(self.height, proportions.vert);

        match (width, height) {
            (Some(width), Some(height)) => Ok(Self  { width,             height              }),
            (Some(width), None        ) => Err(Self { width,             height: self.height }),
            (None,        Some(height)) => Err(Self { width: self.width, height              }),
            (None,        None        ) => Err(self),
        }
    }

    #[inline]
    fn satisfy_p(available: u16, g: P) -> Option<u16>
    {
        match g {
            P::Flexible        => Some(available),
            P::Fixed(v)        => (available >= v).then_some(v),
            P::To(v)           => Some(std::cmp::min(v, available)),
            P::From(v)         => (available >= v).then_some(available),
            P::Range(min, max) => (available >= min)
                .then_some(std::cmp::min(max, available)),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Proportions {
    pub horiz: P,
    pub vert: P,
}

impl Proportions {
    /// Collapse all dimensions to minimum fixed values.
    #[inline]
    pub fn collapse(self) -> Self
    {
        Self {
            horiz: self.horiz.collapse(),
            vert: self.vert.collapse(),
        }
    }

    /// Make the upper ends of all dimensions flexible.
    ///
    /// This creates proportions that can contain the previous value but can
    /// also grow flexibly.
    #[inline]
    pub fn expand(self) -> Self
    {
        Self {
            horiz: self.horiz.expand(),
            vert: self.vert.expand(),
        }
    }

    /// Computes the resulting proportions of 
    ///
    /// Put simply, the resulting proportions express the minimum and maximum
    /// bounds of the original addends. 
    ///
    /// It can be used to express the resulting proportions of 2
    /// [`Proportional`] objects placed next to each other.
    #[inline]
    pub fn add(self, other: Self) -> Self
    {
        Self {
            horiz: self.horiz.add(other.horiz),
            vert:  self.vert.add(other.vert),
        }
    }
}

/// A single proportion.
///
/// This structure defines the **inclusive** ranges that a single dimension of a
/// widget can have.
///
/// NOTE: since a widget can always go as small as it wants to but the max size
/// is the limiting factor, we always assume that the widget wants to be as
/// large as it can (within its specified range).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum P {
    /// Fully flexible.
    #[default]
    Flexible,
    /// Fixed size.
    Fixed(u16),
    /// Flexible, with a fixed maximum.
    ///
    /// NOTE: inclusive.
    To(u16),
    /// Flexible, with a fixed minimum.
    ///
    /// NOTE: inclusive.
    From(u16),
    /// Flexible, with a fixed minimum and maximum.
    ///
    /// NOTE: inclusive.
    Range(u16, u16),
}

impl P {
    /// Get the lower bound.
    #[inline]
    pub fn min(self) -> u16
    {
        match self {
            P::Flexible      => 0,
            P::Fixed(v)      => v,
            P::To(_)         => 0,
            P::From(v)       => v,
            P::Range(min, _) => min,
        }
    }

    /// Get the upper bound (inclusive).
    #[inline]
    pub fn max(self) -> Option<u16>
    {
        match self {
            P::Flexible     => None,
            P::Fixed(v)     => Some(v),
            P::To(to)       => Some(to),
            P::From(_)      => None,
            P::Range(_, to) => Some(to),
        }
    }

    /// Collapse to minimum fixed values.
    #[inline]
    pub fn collapse(self) -> Self
    {
        P::Fixed(self.min())
    }

    /// Make the upper end flexible.
    ///
    /// This creates proportions that can contain the original ones but can also
    /// grow flexibly.
    #[inline]
    pub fn expand(self) -> Self
    {
        match self {
            P::Flexible      => P::Flexible,
            P::Fixed(v)      => P::From(v),
            P::To(_)         => P::Flexible,
            P::From(v)       => P::From(v),
            P::Range(min, _) => P::From(min),
        }
    }

    /// Add the minimum requirements and maximum growth potential.
    ///
    /// Put simply, the resulting proportion expresses the minimum and maximum
    /// bounds of the original addends. 
    ///
    /// It can be used to express the resulting proportions of 2 other
    /// proportions placed next to each other.
    ///
    /// ```
    /// use cwinui::layout::P;
    ///
    /// let a = P::Flexible;
    /// let b = P::Range(3, 45);
    /// assert_eq!(a.add(b).collapse(), P::Fixed(a.min() + b.min()));
    /// assert_eq!(P::From(3).add(P::Fixed(44)).max(), None);
    /// assert_eq!(P::To(3).add(P::Fixed(32)).max(), Some(35));
    /// ```
    #[inline]
    pub fn add(self, other: Self) -> Self
    {
        //let mut start = 0;
        //let mut end   = None;

        //#[inline(always)]
        //fn update_bounds(v: P, start: &mut u16, end: &mut Option<u16>)
        //{
        //    match v {
        //        P::Flexible => { *end = None; },
        //        P::Fixed(v) => {
        //            *start += v;

        //            if let Some(end) = end.as_mut() {
        //                *end += v;
        //            }
        //        },
        //        P::To(to) => {
        //            if let Some(end) = end.as_mut() {
        //                *end += to;
        //            }
        //        },
        //        P::From(from) => {
        //            *start += from;
        //            *end = None;
        //        },
        //        P::Range(from, to) => {
        //            *start += from;
        //            if let Some(end) = end.as_mut() {
        //                *end += to;
        //            }
        //        },
        //    }
        //}

        //update_bounds(self,  &mut start, &mut end);
        //update_bounds(other, &mut start, &mut end);

        //match (start, end) {
        //    (0,    None    )               => P::Flexible,
        //    (0,    Some(to))               => P::To(to),
        //    (from, None    )               => P::From(from),
        //    (from, Some(to)) if from == to => P::Fixed(to),
        //    (from, Some(to))               => P::Range(from, to),
        //}


        match (self, other) {
            (P::Flexible,            P::Flexible)            => P::Flexible,
            (P::Fixed(a),            P::Fixed(b))            => P::Fixed(a + b),
            (P::To(a),               P::To(b))               => P::To(a + b),
            (P::From(a),             P::From(b))             => P::From(a + b),
            (P::Range(a_from, a_to), P::Range(b_from, b_to)) => {
                let from = a_from + b_from;
                let to = a_to + b_to;

                if from == to {
                    P::Fixed(to)
                } else {
                    P::Range(from, to)
                }
            },
            (P::Flexible,   P::To(_))               | (P::To(_),               P::Flexible)    => P::Flexible,
            (P::Flexible,   P::Fixed(v))            | (P::Fixed(v),            P::Flexible)    => P::From(v),
            (P::Flexible,   P::From(from))          | (P::From(from),          P::Flexible)    => P::From(from),
            (P::Flexible,   P::Range(from, _))      | (P::Range(from, _),      P::Flexible)    => P::From(from),
            (P::Fixed(v),   P::From(from))          | (P::From(from),          P::Fixed(v))    => P::From(from + v),
            (P::To(_),      P::From(from))          | (P::From(from),          P::To(_))       => P::From(from),
            (P::From(from), P::Range(o_from, _))    | (P::Range(o_from, _),    P::From(from))  => P::From(from + o_from),
            (P::Fixed(v),   P::To(to))              | (P::To(to),              P::Fixed(v))    => P::Range(v, v + to),
            (P::Fixed(v),   P::Range(from, to))     | (P::Range(from, to),     P::Fixed(v))    => P::Range(v + from, v + to),
            (P::To(to),     P::Range(o_from, o_to)) | (P::Range(o_from, o_to), P::To(to))      => P::Range(o_from, to + o_to),
        }
    }
}

/// Objects that have proportions.
///
/// Types can implement this trait to define their layout requirements.
pub trait Proportional {
    fn proportions(&self) -> Proportions;
}

/// Rectangular area.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    #[inline]
    pub fn from_parts(pos: Pos, dimensions: Dim) -> Self
    {
        Self {
            x: pos.x,
            y: pos.y,
            width: dimensions.width,
            height: dimensions.height,
        }
    }

    #[inline]
    pub fn parts(self) -> (Pos, Dim)
    {
        (
            Pos { x: self.x, y: self.y },
            Dim { width: self.width, height: self.height }
        )
    }

    #[inline]
    pub fn align_to(&self, anchor: Self, alignment: Alignment) -> Self
    {
        let top_left = match alignment {
            Alignment::TopLeft => anchor.top_left(),
            Alignment::TopCenter => Pos {
                x: anchor.center().x.saturating_sub(self.width / 2),
                y: anchor.y,
            },
            Alignment::TopRight => Pos {
                x: anchor.top_right().x.saturating_sub(self.width),
                y: anchor.y,
            },
            Alignment::CenterLeft => Pos {
                x: anchor.x,
                y: anchor.center_left().y.saturating_sub(self.height / 2),
            },
            Alignment::Center => anchor.center()
                .saturating_sub(Pos { x: self.width / 2, y: self.height / 2 }),
            Alignment::CenterRight => anchor.center_right()
                .saturating_sub(Pos { x: self.width, y: self.height / 2 }),
            Alignment::BottomLeft => Pos {
                x: anchor.x,
                y: anchor.bottom_left().y.saturating_sub(self.height / 2),
            },
            Alignment::BottomCenter => anchor.bottom_center()
                .saturating_sub(Pos { x: self.width / 2, y: self.height }),
            Alignment::BottomRight => anchor.bottom_right()
                .saturating_sub(Pos { x: self.width, y: self.height }),
        };

        Self {
            x: top_left.x,
            y: top_left.y,
            width: self.width,
            height: self.height,
        }
    }

    /// Checks if `self` and `other` overlap.
    pub fn overlaps(&self, other: Self) -> bool
    {
        let other_l = other.x;
        let other_r = other.x + other.width;
        let other_t = other.y;
        let other_b = other.y + other.height;

        let self_l = self.x;
        let self_r = self.x + self.width;
        let self_t = self.y;
        let self_b = self.y + self.height;

        let x_overlaps = other_l < self_r && other_r > self_l;
        let y_overlaps = other_t < self_b && other_b > self_t;

        x_overlaps && y_overlaps
    }

    #[inline]
    pub fn contains_pos(&self, pos: Pos) -> bool
    {
        pos.x >= self.x
            && pos.x < self.x + self.width
            && pos.y >= self.y
            && pos.y < self.y + self.height
    }

    /// Checks if either of the dimensions is `0`.
    #[inline]
    pub fn is_void(&self) -> bool
    {
        self.width == 0 || self.height == 0
    }

    /// Returns the area that corresponds to the intersection of `self` and
    /// `other`.
    ///
    /// # Overflows
    ///
    /// When `self` and `other` do not overlap.
    #[inline]
    pub fn intersection(&self, other: Self) -> Self
    {
        let left_x = std::cmp::max(self.x, other.x);
        let right_x = std::cmp::min(self.x + self.width, other.x + other.width);
        let top_y = std::cmp::max(self.y, other.y);
        let bottom_y
            = std::cmp::min(self.y + self.height, other.y + other.height);

        Self {
            x: left_x,
            y: top_y,
            width: right_x - left_x,
            height: bottom_y - top_y,
        }
    }

    #[inline]
    pub fn inset(&self, count: u16) -> Self
    {
        Self {
            x: self.x + count,
            y: self.y + count,
            width: self.width - count * 2,
            height: self.height - count * 2,
        }
    }

    #[inline]
    pub fn split_horiz_at(&self, y: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts.
        assert!(y <= self.height);

        (
            Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height: y,
            },
            Self {
                x: self.x,
                y: self.y + y,
                width: self.width,
                height: self.height - y,
            }
        )
    }

    #[inline]
    pub fn split_vert_at(&self, x: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts.
        assert!(x <= self.width);

        (
            Self {
                x: self.x,
                y: self.y,
                width: x,
                height: self.height,
            },
            Self {
                x: self.x + x,
                y: self.y,
                width: self.width - x,
                height: self.height,
            }
        )
    }

    #[inline]
    pub fn split_horiz_at_abs(&self, y: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts.
        assert!(y >= self.y);
        assert!(y <= self.y + self.height);

        let first_height = y - self.y;

        (
            Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height: first_height,
            },
            Self {
                x: self.x,
                y,
                width: self.width,
                height: self.height - first_height,
            }
        )
    }

    #[inline]
    pub fn split_vert_at_abs(&self, x: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts.
        assert!(x >= self.x);
        assert!(x <= self.x + self.width);

        let first_width = x - self.x;

        (
            Self {
                x: self.x,
                y: self.y,
                width: first_width,
                height: self.height,
            },
            Self {
                x,
                y: self.y,
                width: self.width - first_width,
                height: self.height,
            }
        )
    }

    #[inline]
    pub fn dimensions(&self) -> Dim
    {
        Dim { width: self.width, height: self.height }
    }

    /// Position of the top left corner.
    #[inline]
    pub fn top_left(&self) -> Pos
    {
        Pos { x: self.x, y: self.y }
    }

    /// Position of the midpoint of the top side.
    #[inline]
    pub fn top_center(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y,
        }
    }

    /// Position of the top right corner.
    ///
    /// NOTE: the x coordinate is non-inclusive.
    #[inline]
    pub fn top_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y,
        }
    }

    /// Position of the midpoint of the left side.
    #[inline]
    pub fn center_left(&self) -> Pos
    {
        Pos {
            x: self.x,
            y: self.y + self.height / 2,
        }
    }

    /// Position of the center.
    #[inline]
    pub fn center(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }

    /// Position of the midpoint of the right side.
    ///
    /// NOTE: the x coordinate is non-inclusive.
    #[inline]
    pub fn center_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y + self.height / 2,
        }
    }

    /// Position of the bottom left corner.
    ///
    /// NOTE: the y coordinate is non-inclusive.
    #[inline]
    pub fn bottom_left(&self) -> Pos
    {
        Pos {
            x: self.x,
            y: self.y + self.height,
        }
    }

    /// Position of the midpoint of the bottom side.
    ///
    /// NOTE: the y coordinate is non-inclusive.
    #[inline]
    pub fn bottom_center(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y + self.height,
        }
    }

    /// Position of the bottom right corner.
    ///
    /// NOTE: both coordinates are non-inclusive.
    #[inline]
    pub fn bottom_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y + self.height,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Justify {
    HCenter(u16),
    VCenter(u16),
    Left(u16),
    Right(u16),
    Top(u16),
    Bottom(u16),
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}
