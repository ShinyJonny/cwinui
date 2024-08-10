use crate::util::{min, max};


/// Position coordinates.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, std::hash::Hash)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    /// \[0, 0].
    pub const ZERO: Self = Pos { x: 0, y: 0 };

    /// Performs a saturating addition.
    ///
    /// ```
    /// use cwinui::Pos;
    ///
    /// let a = Pos { x: 4, y: 3 };
    /// let b = Pos { x: 1, y: 9 };
    /// assert_eq!(
    ///     a.saturating_add(b),
    ///     Pos {
    ///         x: a.x.saturating_add(b.x),
    ///         y: a.y.saturating_add(b.y),
    ///     }
    /// );
    /// ```
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self
    {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }

    /// Performs a saturating subtraction.
    ///
    /// ```
    /// use cwinui::Pos;
    ///
    /// let a = Pos { x: 4, y: 3 };
    /// let b = Pos { x: 1, y: 9 };
    /// assert_eq!(
    ///     a.saturating_sub(b),
    ///     Pos {
    ///         x: a.x.saturating_sub(b.x),
    ///         y: a.y.saturating_sub(b.y),
    ///     }
    /// );
    /// ```
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self
    {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    /// Adds `x` to `self.x`.
    #[inline]
    pub const fn add_x(self, x: u16) -> Self
    {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }

    /// Adds `y` to `self.y`.
    #[inline]
    pub const fn add_y(self, y: u16) -> Self
    {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }

    /// Subtracts `x` from `self.x`.
    #[inline]
    pub const fn sub_x(self, x: u16) -> Self
    {
        Self {
            x: self.x - x,
            y: self.y,
        }
    }

    /// Subtracts `y` from `self.y`.
    #[inline]
    pub const fn sub_y(self, y: u16) -> Self
    {
        Self {
            x: self.x,
            y: self.y - y,
        }
    }

    /// Const version of `Add::add`.
    #[inline]
    pub const fn add(self, rhs: Self) -> Self
    {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    /// Const version of `Sub::sub`.
    #[inline]
    pub const fn sub(self, rhs: Self) -> Self
    {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self::add(self, rhs)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output
    {
        Self::sub(self, rhs)
    }
}

/// Area dimensions.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, std::hash::Hash)]
pub struct Dim {
    pub width: u16,
    pub height: u16,
}

impl Dim {
    /// Checks if either of the dimensions is `0`.
    #[inline]
    pub const fn is_collapsed(self) -> bool
    {
        self.width == 0 || self.height == 0
    }

    /// Checks if the dimensions can fit into `proportions`.
    #[inline]
    pub const fn satisfies(self, proportions: Proportions) -> bool
    {
        self.width >= proportions.width.min
            && self.height >= proportions.height.min
    }

    /// Try to fit the dimensions into the [`Proportions`].
    ///
    /// This method will always attempt to yield as large dimensions as the
    /// proportions allow. If the dimensions can't fit into the proportions,
    /// the `Err` value is returned with the best possible attempt at fitting
    /// into the proportions.
    #[inline]
    pub const fn fit_into(self, p: Proportions) -> Result<Dim, Dim>
    {
        let width  = Self::fit_range(self.width, p.width);
        let height = Self::fit_range(self.height, p.height);

        match (width, height) {
            (Some(width), Some(height)) => Ok(Dim  { width,             height              }),
            (Some(width), None        ) => Err(Dim { width,             height: self.height }),
            (None,        Some(height)) => Err(Dim { width: self.width, height              }),
            (None,        None        ) => Err(self),
        }
    }

    #[inline(always)]
    const fn fit_range(available: u16, range: Range) -> Option<u16>
    {
        if available < range.min {
            return None;
        }

        let provision = if let Some(max) = range.max {
            min!(available, max)
        } else {
            available
        };

        Some(provision)
    }
}

/// Proportions of widgets that can be laid out in space.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, std::hash::Hash)]
pub struct Proportions {
    pub width: Range,
    pub height: Range,
}

impl From<Dim> for Proportions {
    fn from(dim: Dim) -> Self
    {
        Self {
            width: Range::fixed(dim.width),
            height: Range::fixed(dim.height),
        }
    }
}

impl Proportions {
    /// Both `horiz` and `vert` have the range of `0..=0` .
    pub const ZERO: Self = Self {
        width:  Range::ZERO,
        height: Range::ZERO,
    };

    /// Creates fixed proportions from [`Dim`].
    pub const fn fixed(dim: Dim) -> Self
    {
        Self {
            width: Range::fixed(dim.width),
            height: Range::fixed(dim.height),
        }
    }

    /// Creates fully flexible proportions, i.e. both `horiz` and `vert` are
    /// `0..`
    pub const fn flexible() -> Self
    {
        Self {
            width: Range::flexible(),
            height: Range::flexible(),
        }
    }

    /// Collapses all dimensions to minimum fixed values.
    #[inline]
    pub const fn collapse(self) -> Self
    {
        Self {
            width: self.width.collapse(),
            height: self.height.collapse(),
        }
    }

    /// Makes the upper ends of all dimensions flexible.
    ///
    /// This creates proportions that can contain the previous ones but can also
    /// grow flexibly.
    #[inline]
    pub const fn expand(self) -> Self
    {
        Self {
            width: self.width.expand(),
            height: self.height.expand(),
        }
    }

    /// Adds the range requirements for both directions.
    #[inline]
    pub const fn add(self, other: Self) -> Self
    {
        Self {
            width: self.width.add(other.width),
            height: self.height.add(other.height),
        }
    }

    /// Joins the range requirements for both directions.
    #[inline]
    pub const fn join(self, other: Self) -> Self
    {
        Self {
            width: self.width.join(other.width),
            height: self.height.join(other.height),
        }
    }
}

/// Inclusive range of sizes.
///
/// This structure defines the **inclusive** ranges that a single dimension of a
/// widget can have.
///
/// NOTE: since a widget can always go as small as it wants to but the max size
/// is the limiting factor, we always assume that the widget wants to be as
/// large as it can (within its specified range).
#[derive(Copy, Clone, Default, PartialEq, Eq, std::hash::Hash)]
pub struct Range {
    min: u16,
    max: Option<u16>,
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        <u16 as std::fmt::Debug>::fmt(&self.min, f)?;
        f.write_str("..")?;
        if let Some(max) = self.max {
            f.write_str("=")?;
            <u16 as std::fmt::Debug>::fmt(&max, f)?;
        }

        Ok(())
    }
}

impl Range {
    /// `0..=0`
    pub const ZERO: Self = Self::fixed(0);

    // NOTE: should we panic when max < min?
    /// Creates a new range `min..=max`.
    ///
    /// If `max` is less than `min`, it is ignored and `min` becomes the max
    /// value (`min..=min`).
    pub const fn new(min: u16, max: u16) -> Self
    {
        Self {
            min,
            max: Some(max!(min, max)),
        }
    }

    /// Creates a new range without any checks.
    ///
    /// # Safety
    ///
    /// It is unsound to call this function with `max < min`.
    pub const unsafe fn new_unchecked(min: u16, max: u16) -> Self
    {
        Self { min, max: Some(max) }
    }

    /// Creates a new range from its raw parts without any checks.
    ///
    /// # Safety
    ///
    /// It is unsound to call this function with `Some(max) < min`.
    pub const unsafe fn from_raw_parts(min: u16, max: Option<u16>) -> Self
    {
        Self { min, max }
    }

    /// Creates a fixed range (`size..=size`).
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let r = Range::fixed(4);
    ///
    /// assert_eq!(r.min(), 4);
    /// assert_eq!(r.max(), Some(4));
    /// ```
    pub const fn fixed(size: u16) -> Self
    {
        Self {
            min: size,
            max: Some(size),
        }
    }

    /// Creates a flexible range starting at `size` (`size..`).
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let r = Range::from(4);
    ///
    /// assert_eq!(r.min(), 4);
    /// assert_eq!(r.max(), None);
    /// ```
    pub const fn from(size: u16) -> Self
    {
        Self {
            min: size,
            max: None,
        }
    }

    /// Creates a range ending at `size` (`0..=size`).
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let r = Range::to(4);
    ///
    /// assert_eq!(r.min(), 0);
    /// assert_eq!(r.max(), Some(4));
    /// ```
    pub const fn to(size: u16) -> Self
    {
        Self {
            min: 0,
            max: Some(size),
        }
    }

    /// Creates a fully flexible range (`0..`).
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let r = Range::flexible();
    ///
    /// assert_eq!(r.min(), 0);
    /// assert_eq!(r.max(), None);
    /// ```
    pub const fn flexible() -> Self
    {
        Self {
            min: 0,
            max: None,
        }
    }

    /// The low bound.
    pub const fn min(self) -> u16
    {
        self.min
    }

    /// The upper bound.
    pub const fn max(self) -> Option<u16>
    {
        self.max
    }

    /// Collapse the maximum to be equal to the minimum.
    #[inline]
    pub const fn collapse(mut self) -> Self
    {
        self.max = Some(self.min);

        self
    }

    /// Make the upper end flexible.
    #[inline]
    pub const fn expand(mut self) -> Self
    {
        self.max = None;

        self
    }

    /// Add the minimum requirements and maximum growth potential.
    ///
    /// The minimums and maximums are added.
    ///
    /// Can be used to express the result of placing 2 ranges next to each
    /// other.
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let a = Range::from(3);
    /// let b = Range::new(7, 7);
    /// let c = Range::new(2, 33);
    ///
    /// assert_eq!(a.add(b), Range::from(10));
    /// assert_eq!(b.add(c), Range::new(9, 40));
    /// ```
    #[inline]
    pub const fn add(self, other: Self) -> Self
    {
        Self {
            min: self.min + other.min,
            max: if let (Some(a), Some(b)) = (self.max, other.max) {
                Some(a + b)
            } else {
                None
            },
        }
    }

    /// Joins the ranges.
    ///
    /// The resulting minimum and maximum are the higher ones.
    ///
    /// ```
    /// use cwinui::layout::Range;
    ///
    /// let a = Range::from(7);
    /// let b = Range::new(2, 7);
    /// let c = Range::new(3, 33);
    ///
    /// assert_eq!(a.join(b), Range::from(7));
    /// assert_eq!(b.join(c), Range::new(3, 33));
    /// ```
    #[inline]
    pub const fn join(self, other: Self) -> Self
    {
        Self {
            min: max!(self.min, other.min),
            max: if let (Some(a), Some(b)) = (self.max, other.max) {
                Some(max!(a, b))
            } else {
                None
            },
        }
    }
}

/// Objects that have proportions.
///
/// Types can implement this trait to define their layout requirements.
pub trait Proportional {
    /// Computes the proportions.
    fn proportions(&self) -> Proportions;
}

impl<T: Proportional> Proportional for &T {
    fn proportions(&self) -> Proportions
    {
        T::proportions(*self)
    }
}

impl<T: Proportional> Proportional for &mut T {
    fn proportions(&self) -> Proportions
    {
        T::proportions(*self)
    }
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
    /// Creates `Area` from the position of the top-left corner and its
    /// dimensions.
    #[inline]
    pub const fn from_parts(pos: Pos, dimensions: Dim) -> Self
    {
        Self {
            x: pos.x,
            y: pos.y,
            width: dimensions.width,
            height: dimensions.height,
        }
    }

    /// Gets the position of the top-left corner and the dimensions.
    #[inline]
    pub const fn parts(self) -> (Pos, Dim)
    {
        (
            Pos { x: self.x, y: self.y },
            Dim { width: self.width, height: self.height }
        )
    }

    /// Aligns `self` to `anchor`.
    #[inline]
    pub const fn align_to(&self, anchor: Self, alignment: Alignment) -> Self
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
                y: anchor.bottom_left().y.saturating_sub(self.height),
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

    /// Checks if areas overlap.
    pub const fn overlaps(&self, other: Self) -> bool
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

    /// Checks if `pos` is falls within the area.
    #[inline]
    pub const fn contains_pos(&self, pos: Pos) -> bool
    {
        pos.x >= self.x
            && pos.x < self.x + self.width
            && pos.y >= self.y
            && pos.y < self.y + self.height
    }

    /// Checks if either of the dimensions is `0`.
    #[inline]
    pub const fn is_collapsed(&self) -> bool
    {
        self.width == 0 || self.height == 0
    }

    /// Computes the intersection of `self` and `other`.
    ///
    /// **It is unsound to call this on areas that do not overlap!!!**
    ///
    /// # Overflows
    ///
    /// When `self` and `other` do not overlap.
    #[inline]
    pub const fn intersection(&self, other: Self) -> Self
    {
        let left_x   = max!(self.x, other.x);
        let right_x  = min!(self.x + self.width, other.x + other.width);
        let top_y    = max!(self.y, other.y);
        let bottom_y = min!(self.y + self.height, other.y + other.height);

        Self {
            x: left_x,
            y: top_y,
            width: right_x - left_x,
            height: bottom_y - top_y,
        }
    }

    /// Shrinks the area from each side by `count`.
    ///
    /// # Underflows
    ///
    /// When `width` or `height` are less than `count * 2`.
    #[inline]
    pub const fn inset(&self, count: u16) -> Self
    {
        Self {
            x: self.x + count,
            y: self.y + count,
            width: self.width - count * 2,
            height: self.height - count * 2,
        }
    }

    /// Splits the area horizontally at `y` relative to the start of the area.
    ///
    /// # Panics
    ///
    /// When `y` is greater than the height.
    #[inline]
    pub const fn split_horiz_at(&self, y: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts?
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

    /// Splits the area vertically at `x` relative to the start of the area.
    ///
    /// # Panics
    ///
    /// When `x` is greater than the width.
    #[inline]
    pub const fn split_vert_at(&self, x: u16) -> (Self, Self)
    {
        // FIXME: make these debug asserts?
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

    /// Splits the area horizontally at `y`.
    ///
    /// # Panics
    ///
    /// When `y` is not contained in the area.
    #[inline]
    pub const fn split_horiz_at_abs(&self, y: u16) -> (Self, Self)
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

    /// Splits the area vertically at `x`.
    ///
    /// # Panics
    ///
    /// When `x` is not contained in the area.
    #[inline]
    pub const fn split_vert_at_abs(&self, x: u16) -> (Self, Self)
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

    /// Dimensions of the area.
    #[inline]
    pub const fn dimensions(&self) -> Dim
    {
        Dim { width: self.width, height: self.height }
    }

    /// Position of the top left corner.
    #[inline]
    pub const fn top_left(&self) -> Pos
    {
        Pos { x: self.x, y: self.y }
    }

    /// Position of the midpoint of the top side.
    #[inline]
    pub const fn top_center(&self) -> Pos
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
    pub const fn top_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y,
        }
    }

    /// Position of the midpoint of the left side.
    #[inline]
    pub const fn center_left(&self) -> Pos
    {
        Pos {
            x: self.x,
            y: self.y + self.height / 2,
        }
    }

    /// Position of the center.
    #[inline]
    pub const fn center(&self) -> Pos
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
    pub const fn center_right(&self) -> Pos
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
    pub const fn bottom_left(&self) -> Pos
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
    pub const fn bottom_center(&self) -> Pos
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
    pub const fn bottom_right(&self) -> Pos
    {
        Pos {
            x: self.x + self.width,
            y: self.y + self.height,
        }
    }
}

/// Alignment of an item within a rectangle.
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

/// Alignment of a string of items within a rectangle.
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
