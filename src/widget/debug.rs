use crate::{Area, Pos};
use crate::style::WithStyle;
use crate::layout::{Justify, Proportional, Proportions};

use super::{border, Border, Paint, Widget};


/// Option flags for [`Wireframe`].
#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub outline: bool,
    pub corners: bool,
    pub size: bool,
    pub midpoints: bool,
    pub center: bool,
    pub diagonals: bool,
}

impl Flags {
    /// Const version of `Default::default`.
    #[inline]
    pub const fn default() -> Self
    {
        Self {
            outline: true,
            size: true,
            center: true,
            corners: true,
            diagonals: true,
            midpoints: true,
        }
    }
}

impl Default for Flags {
    #[inline]
    fn default() -> Self
    {
        Self::default()
    }
}


/// Wireframe of the render area.
///
/// Can show debugging information such as the outline of the render area and
/// the size of the render area.
#[derive(Debug, Clone)]
pub struct Wireframe {
    pub flags: Flags,
}

impl Wireframe {
    /// Creates a new `Wireframe` with default options.
    pub const fn new() -> Self
    {
        Self {
            flags: Flags::default(),
        }
    }

    /// Adjusts the option to show the border.
    #[inline]
    pub const fn outline(mut self, flag: bool) -> Self
    {
        self.flags.outline = flag;

        self
    }

    /// Adjusts the option to show the size.
    #[inline]
    pub const fn size(mut self, flag: bool) -> Self
    {
        self.flags.size = flag;

        self
    }

    /// Adjusts the option to show the center.
    #[inline]
    pub const fn center(mut self, flag: bool) -> Self
    {
        self.flags.center = flag;

        self
    }

    /// Adjusts the option to show the corners.
    #[inline]
    pub const fn corners(mut self, flag: bool) -> Self
    {
        self.flags.corners = flag;

        self
    }

    /// Adjusts the option to show the diagonal lines.
    #[inline]
    pub const fn diagonals(mut self, flag: bool) -> Self
    {
        self.flags.diagonals = flag;

        self
    }

    /// Adjusts the option to show the midpoints.
    #[inline]
    pub const fn midpoints(mut self, flag: bool) -> Self
    {
        self.flags.midpoints = flag;

        self
    }
}

impl<P: Paint> Widget<P> for Wireframe {
    fn render(&self, buf: &mut P, area: crate::Area)
    {
        if area.is_collapsed() {
            return;
        }

        let corner = '+'.styled();
        let vbar = '|'.styled();
        let hbar = '-'.styled();
        let center = 'x'.styled();

        if self.flags.outline {
            Border::new(super::Void)
                .theme(border::Theme {
                    top_left: corner,
                    top_right: corner,
                    bottom_right: corner,
                    bottom_left: corner,
                    top: hbar,
                    right: vbar,
                    bottom: hbar,
                    left: vbar,
                })
                .render(buf, area);
        }

        if self.flags.midpoints {
            let width_is_even  = area.width & 1 == 0;
            let height_is_even = area.height & 1 == 0;

            let w_mid = (area.width  - 1) / 2;
            let h_mid = (area.height - 1) / 2;

            if width_is_even {
                buf.jprint("\\/", Justify::Top(w_mid),    area);
                buf.jprint("/\\", Justify::Bottom(w_mid), area);
            } else {
                buf.jputc('v', Justify::Top(w_mid),    area);
                buf.jputc('^', Justify::Bottom(w_mid), area);
            }

            if height_is_even {
                buf.jputc('\\', Justify::Left(h_mid),      area);
                buf.jputc('/',  Justify::Left(h_mid + 1),  area);
                buf.jputc('/',  Justify::Right(h_mid),     area);
                buf.jputc('\\', Justify::Right(h_mid + 1), area);
            } else {
                buf.jputc('>', Justify::Left(h_mid),    area);
                buf.jputc('<', Justify::Right(h_mid), area);
            }
        }

        if self.flags.corners {
            buf.jputc(corner, Justify::TopLeft, area);
            buf.jputc(corner, Justify::TopRight, area);
            buf.jputc(corner, Justify::BottomLeft, area);
            buf.jputc(corner, Justify::BottomRight, area);
        }
        if self.flags.diagonals {
        }
        if self.flags.center {
            let width_is_even  = area.width & 1 == 0;
            let height_is_even = area.height & 1 == 0;

            match (width_is_even, height_is_even) {
                (true, true)   => {
                    let pos = Pos {
                        x: (area.width - 1) / 2,
                        y: (area.height - 1) / 2,
                    };
                    buf.putc(pos,                   '\\', area);
                    buf.putc(pos.add(Pos{x:1,y:1}), '\\', area);
                    buf.putc(pos.add_x(1),          '/' , area);
                    buf.putc(pos.add_y(1),          '/' , area);
                },
                (true, false)  => {
                    buf.jprint("><", Justify::Center, area);
                },
                (false, true)  => {
                    buf.jputc('v', Justify::HCenter((area.height -1) / 2), area);
                    buf.jputc('^', Justify::HCenter(area.height / 2), area);
                },
                (false, false) => {
                    buf.jputc(center, Justify::Center, area);
                },
            }
        }
        if self.flags.size {
            // FIXME: use an array string to make this allocation-free.
            let size = format!("[{}x{}]", area.width, area.height);
            buf.jprint(size.as_str(), Justify::TopLeft, Area {
                x: area.x + 1,
                y: area.y,
                width: area.width.saturating_sub(2),
                height: area.height,
            })
        }
    }
}

impl Proportional for Wireframe {
    #[inline]
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}
