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
        let width  = Self::satisfy_range(self.width, proportions.horiz);
        let height = Self::satisfy_range(self.height, proportions.vert);

        match (width, height) {
            (Some(width), Some(height)) => Ok(Self  { width,             height              }),
            (Some(width), None        ) => Err(Self { width,             height: self.height }),
            (None,        Some(height)) => Err(Self { width: self.width, height              }),
            (None,        None        ) => Err(self),
        }
    }

    #[inline(always)]
    fn satisfy_range(available: u16, range: Range) -> Option<u16>
    {
        if available >= range.min {
            Some(range.max.map(|max| std::cmp::min(available, max))
                .unwrap_or(available))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Proportions {
    pub horiz: Range,
    pub vert: Range,
}

impl Proportions {
    pub const ZERO: Self = Self {
        horiz: Range::ZERO,
        vert:  Range::ZERO,
    };

    pub const fn fixed(dim: Dim) -> Self
    {
        Self {
            horiz: Range::fixed(dim.width),
            vert:  Range::fixed(dim.height),
        }
    }

    pub const fn flexible() -> Self
    {
        Self {
            horiz: Range::flexible(),
            vert:  Range::flexible(),
        }
    }

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

    #[inline]
    pub fn union(self, other: Self) -> Self
    {
        Self {
            horiz: self.horiz.union(other.horiz),
            vert:  self.vert.union(other.vert),
        }
    }
}

/// A range of sizes.
///
/// This structure defines the **inclusive** ranges that a single dimension of a
/// widget can have.
///
/// NOTE: since a widget can always go as small as it wants to but the max size
/// is the limiting factor, we always assume that the widget wants to be as
/// large as it can (within its specified range).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Range {
    pub min: u16,
    pub max: Option<u16>,
}

impl Range {
    pub const ZERO: Self = Self::fixed(0);

    pub const fn fixed(size: u16) -> Self
    {
        Self {
            min: size,
            max: Some(size),
        }
    }

    pub const fn from(size: u16) -> Self
    {
        Self {
            min: size,
            max: None,
        }
    }

    pub const fn to(size: u16) -> Self
    {
        Self {
            min: 0,
            max: Some(size),
        }
    }

    pub const fn flexible() -> Self
    {
        Self {
            min: 0,
            max: None,
        }
    }

    /// Collapse the maximum to be equal to the minimum.
    #[inline]
    pub fn collapse(mut self) -> Self
    {
        self.max = Some(self.min);

        self
    }

    /// Make the upper end flexible.
    ///
    /// This creates proportions that can contain the original ones but can also
    /// grow flexibly.
    #[inline]
    pub fn expand(mut self) -> Self
    {
        self.max = None;

        self
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
        Self {
            min: self.min + other.min,
            max: Option::zip(self.max, other.max)
                .map(|(a, b)| a + b),
        }
    }

    #[inline]
    pub fn union(self, other: Self) -> Self
    {
        Self {
            min: std::cmp::max(self.min, other.min),
            max: Option::zip(self.max, other.max)
                .map(|(a, b)| std::cmp::max(a, b)),
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
