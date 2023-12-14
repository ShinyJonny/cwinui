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
    /// Return `Dim` that satisfies `proportions`.
    ///
    /// This method will always yield as large dimensions as `proportions`
    /// allow.
    pub fn satisfy(self, proportions: Proportions) -> Result<Self, Self>
    {
        let width = Self::satisfy_g(self.width, proportions.horiz);
        let height = Self::satisfy_g(self.height, proportions.vert);

        match (width, height) {
            (None,        None        ) => Err(self),
            (None,        Some(height)) => Err(Self { width: self.width, height }),
            (Some(width), None        ) => Err(Self { width, height: self.height }),
            (Some(width), Some(height)) => Ok(Self { width, height }),
        }
    }

    fn satisfy_g(available: u16, g: G) -> Option<u16>
    {
        match g {
            G::Flexible        => Some(available),
            G::Fixed(v)        => (available >= v).then_some(v),
            G::To(v)           => Some(std::cmp::min(v, available)),
            G::From(v)         => (available >= v).then_some(available),
            G::Range(min, max) => (available >= min)
                .then_some(std::cmp::min(max, available)),
            G::Max             => Some(available),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Proportions {
    pub horiz: G,
    pub vert: G,
}

impl Proportions {
    /// Collapse all dimensions to minimum fixed values.
    pub fn min(self) -> Self
    {
        Self {
            horiz: self.horiz.min(),
            vert: self.vert.min(),
        }
    }

    /// Raise all dimensions to maximum fixed values.
    pub fn max(self) -> Self
    {
        Self {
            horiz: self.horiz.max(),
            vert: self.vert.max(),
        }
    }

    /// Make the upper ends of all dimensions flexible.
    ///
    /// This creates a geometry that can contain the previous value but can also
    /// grow flexibly.
    pub fn expand(self) -> Self
    {
        Self {
            horiz: self.horiz.expand(),
            vert: self.vert.expand(),
        }
    }
}

/// Geometry of a single dimension in `Proportions`.
///
/// NOTE: since a widget can always go as small as it wants to but the max size
/// is the limiting factor, we always assume that the widget wants to be as
/// large as it can (within its specified range).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum G {
    /// Fully flexible.
    #[default]
    Flexible,
    /// Fixed size.
    Fixed(u16),
    /// Flexible, with a fixed maximum.
    ///
    /// NOTE: inclusive
    To(u16),
    /// Flexible, with a fixed minimum.
    From(u16),
    /// Flexible, with a fixed minimum and maximum.
    ///
    /// NOTE: inclusive
    Range(u16, u16),
    /// The maximum available value.
    ///
    /// This value is to be treated as a fixed value of the maximum possible
    /// value.
    Max,
}

impl G {
    /// Collapse to minimum fixed values.
    #[inline]
    pub fn min(self) -> Self
    {
        match self {
            G::Flexible      => G::Fixed(0),
            G::Fixed(v)      => G::Fixed(v),
            G::To(_)         => G::Fixed(0),
            G::From(v)       => G::Fixed(v),
            G::Range(min, _) => G::Fixed(min),
            G::Max           => G::Max,
        }
    }

    /// Raise to maximum fixed values.
    #[inline]
    pub fn max(self) -> Self
    {
        match self {
            G::Flexible      => G::Max,
            G::Fixed(v)      => G::Fixed(v),
            G::To(v)         => G::Fixed(v),
            G::From(_)       => G::Max,
            G::Range(_, max) => G::Fixed(max),
            G::Max           => G::Max,
        }
    }

    /// Make the upper end flexible.
    ///
    /// This creates a geometry that can contain the previous value but can also
    /// grow flexibly.
    #[inline]
    pub fn expand(self) -> Self
    {
        match self {
            G::Flexible      => G::Flexible,
            G::Fixed(v)      => G::From(v),
            G::To(_)         => G::Flexible,
            G::From(v)       => G::From(v),
            G::Range(min, _) => G::From(min),
            G::Max           => G::Max,
        }
    }
}

/// Objects that have proportions.
///
/// Types can implement this trait to define their proportion requirements.
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

    pub fn align_to(&self, anchor: Self, a: Alignment) -> Self
    {
        let top_left = match a {
            Alignment::TopLeft => anchor.top_left(),
            Alignment::TopCentre => Pos {
                x: anchor.centre().x.saturating_sub(self.width / 2),
                y: anchor.y,
            },
            Alignment::TopRight => Pos {
                x: anchor.top_right().x.saturating_sub(self.width),
                y: anchor.y,
            },
            Alignment::CentreLeft => Pos {
                x: anchor.x,
                y: anchor.centre_left().y.saturating_sub(self.height / 2),
            },
            Alignment::Centre => anchor.centre()
                .saturating_sub(Pos { x: self.width / 2, y: self.height / 2 }),
            Alignment::CentreRight => anchor.centre_right()
                .saturating_sub(Pos { x: self.width, y: self.height / 2 }),
            Alignment::BottomLeft => Pos {
                x: anchor.x,
                y: anchor.bottom_left().y.saturating_sub(self.height / 2),
            },
            Alignment::BottomCentre => anchor.bottom_centre()
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
    #[inline]
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

    #[inline]
    pub fn top_left(&self) -> Pos
    {
        Pos { x: self.x, y: self.y }
    }

    #[inline]
    pub fn top_centre(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y,
        }
    }

    #[inline]
    pub fn top_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y,
        }
    }

    #[inline]
    pub fn centre_left(&self) -> Pos
    {
        Pos {
            x: self.x,
            y: self.y + self.height / 2,
        }
    }

    /// The coordinates of the centre of the area.
    #[inline]
    pub fn centre(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }

    #[inline]
    pub fn centre_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y + self.height / 2,
        }
    }

    #[inline]
    pub fn bottom_left(&self) -> Pos
    {
        Pos {
            x: self.x,
            y: self.y + self.height,
        }
    }

    #[inline]
    pub fn bottom_centre(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y + self.height,
        }
    }

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
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Justify {
    HCentre(u16),
    VCentre(u16),
    Left(u16),
    Right(u16),
    Top(u16),
    Bottom(u16),
    #[default]
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}
