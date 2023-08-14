pub enum Justify {
    HCentre(u32),
    VCentre(u32),
    Left(u32),
    Right(u32),
    Top(u32),
    Bottom(u32),
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

pub enum Align {
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

pub fn align(
    a: Align,
    follower_height: usize,
    follower_width: usize,
    anchor_y: u32,
    anchor_x: u32,
    anchor_height: usize,
    anchor_width: usize
) -> (u32, u32)
{
    let y: u32;
    let x: u32;

    match a {
        Align::TopLeft => {
            x = anchor_x;
            y = anchor_y;
        },
        Align::TopCentre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            y = anchor_y;
        },
        Align::TopRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            y = anchor_y;
        },
        Align::CentreLeft => {
            x = anchor_x;

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::Centre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::CentreRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32 / 2;
            }
        },
        Align::BottomLeft => {
            x = anchor_x;

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
        Align::BottomCentre => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32 / 2;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
        Align::BottomRight => {
            if follower_width >= anchor_width {
                x = anchor_x;
            } else {
                x = anchor_x + (anchor_width - follower_width) as u32;
            }

            if follower_height >= anchor_height {
                y = anchor_y;
            } else {
                y = anchor_y + (anchor_height - follower_height) as u32;
            }
        },
    }

    (y, x)
}

/// Position coordinates.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

/// Rectangular area, used for layouting.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    /// The coordinates of the center of the area.
    pub fn centre(&self) -> Pos
    {
        Pos {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }

    // TODO: revise the signature and naming:
    // - do we mutate or create a new struct?
    pub fn align_centres(&self, anchor: Self) -> Self { todo!(); } // TODO

    pub fn align_to(&self, anchor: Self, align: Align) -> Self { todo!(); } // TODO

    pub fn inset(&self, count: u16) -> Self
    {
        Self {
            x: self.x + count,
            y: self.y + count,
            width: self.width - count * 2,
            height: self.height - count * 2,
        }
    }
}
