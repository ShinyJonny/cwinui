use crate::render::{Render, Draw};
use crate::style::{AsStyledStr, WithStyle};
use crate::layout::{Pos, Proportional, Proportions, Range};

#[allow(unused_imports)]
use crate::style::StyledStr;


/// A single [`StyledStr`] displayed on one line.
///
/// Does not wrap.
pub struct Line<T: AsStyledStr>(pub T);

impl<T: AsStyledStr, R: Render> Draw<R> for Line<T> {
    fn draw(&self, buf: &mut R, area: crate::Area)
    {
        buf.print(Pos::ZERO, self.0.as_styled_str(), area);
    }
}

impl<T: AsStyledStr> Proportional for Line<T> {
    fn proportions(&self) -> Proportions
    {
        Proportions {
            height: Range::fixed(1),
            // NOTE: potential overflow.
            // TODO: utf-8 support.
            width: Range::fixed(self.0.as_styled_str().content.len() as u16),
        }
    }
}


// TODO: wrapping methods.
/// A wrapping [`StyledStr`].
///
/// Due to wrapping, the proportions do not have a fixed value and are `1..` on
/// both axes.
pub struct WrapLine<T: AsStyledStr>(pub T);

impl<T: AsStyledStr, R: Render> Draw<R> for WrapLine<T> {
    fn draw(&self, buf: &mut R, area: crate::Area)
    {
        if area.is_collapsed() {
            return;
        }

        let s = self.0.as_styled_str();
        let vert_size = std::cmp::min(
            area.height as usize,
            // TODO: utf-8
            s.content.len().div_ceil(area.width as usize)
        );

        for y in 0..vert_size {
            let offset = y * area.width as usize;
            let slice = s.slice(offset..);
            buf.print(Pos { x: 0, y: y as u16 }, slice, area);
        }
    }
}

impl<T: AsStyledStr> Proportional for WrapLine<T> {
    fn proportions(&self) -> Proportions
    {
        Proportions {
            height: Range::from(1),
            width: Range::from(1),
        }
    }
}


/// Multiple [`StyledStr`]s chained on one line.
///
/// Does not wrap.
pub struct Chain<'a, T: AsStyledStr>(pub &'a [T]);

impl<'a, T: AsStyledStr, R: Render> Draw<R> for Chain<'a, T> {
    fn draw(&self, buf: &mut R, area: crate::Area)
    {
        let mut offset = 0;

        for link in self.0 {
            let link = link.as_styled_str();
            buf.print(Pos { x: offset as u16, y: 0 }, link, area);

            // TODO: utf-8
            offset += link.content.len();

            if offset >= area.width as usize { break }
        }
    }
}

impl<'a, T: AsStyledStr> Proportional for Chain<'a, T> {
    fn proportions(&self) -> Proportions
    {
        let len = self.0.iter()
            // TODO: utf-8
            .map(|link| link.as_styled_str().content.len())
            .sum();
        let len = std::cmp::min(len, u16::MAX as usize) as u16;

        Proportions {
            width: Range::fixed(len),
            height: Range::fixed(1),
        }
    }
}


// TODO: wrapping methods.
/// Multiple [`StyledStr`]s chained on one line.
///
/// Due to wrapping, the proportions do not have a fixed value and are `1..` on
/// both axes.
pub struct WrapChain<'a, T: AsStyledStr>(pub &'a [T]);

impl<'a, T: AsStyledStr, R: Render> Draw<R> for WrapChain<'a, T> {
    fn draw(&self, buf: &mut R, area: crate::Area)
    {
        if area.is_collapsed() {
            return;
        }

        let mut x = 0;
        let mut y = 0;

        'root: for link in self.0.iter() {
            let link = link.as_styled_str();
            let mut remaining = link.content;

            while remaining.len() > 0 {
                let available = (area.width - x) as usize;
                // TODO: utf-8
                let print_len = std::cmp::min(available, remaining.len());
                let to_print;
                (to_print, remaining) = remaining.split_at(print_len);

                let line = to_print.with_style(|_| link.style);
                buf.print(Pos { x, y }, line, area);

                x += print_len as u16;
                if print_len == available {
                    x = 0;
                    y += 1;

                    if y == area.height {
                        break 'root;
                    }
                }
            }
        }
    }
}

impl<'a, T: AsStyledStr> Proportional for WrapChain<'a, T> {
    fn proportions(&self) -> Proportions
    {
        Proportions {
            width: Range::from(1),
            height: Range::from(1),
        }
    }
}
