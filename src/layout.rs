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

pub trait Aligned {
    fn inner_width(&self) -> usize;
    fn inner_height(&self) -> usize;
    fn inner_start_yx(&self) -> (u32, u32);
    fn outer_width(&self) -> usize;
    fn outer_height(&self) -> usize;
    fn outer_start_yx(&self) -> (u32, u32);
    fn centre(&self) -> (u32, u32);
    fn align_centres<T: Aligned>(&mut self, anchor: &T);
    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align);
    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align);
    fn adjust_pos(&mut self, y: i32, x: i32);
    fn change_pos(&mut self, y: u32, x: u32);
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
