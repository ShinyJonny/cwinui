//! Flexbox-like containers.
//!
//! Each contained item gets its minimum proportion requirements (if possible).
//! The rest of the paint area is distributed among the flexible items equally,
//! in proportion to the size of their request compared to other flexible items.
//!
//! In case the minimum proportion requirements cannot be met, the items are
//! drawn sequentially until there is no space left.
//!
//! Flexible items whose maximum exceeds the paint area or have no maximum are
//! truncated to the 100% of the paint area.


use super::Draw;
use crate::widget::Paint;
use crate::layout::{Proportional, Proportions, Range};
use crate::Area;


/// Items that can be drawn in a *flex container*.
pub trait FlexItem<P: Paint>: Draw<P> + Proportional {}

impl<P: Paint, T> FlexItem<P> for T
where
    T: Draw<P> + Proportional {}


/// Vertical flex container.
///
/// For more information on how the items are drawn, see the [Module-level
/// documentation](self).
#[derive(Clone)]
pub struct FlexCol<'a, P: Paint>(pub &'a [&'a dyn FlexItem<P>]);

impl<'a, P: Paint> std::fmt::Debug for FlexCol<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str("FlexCol ")?;
        f.debug_list()
            .entries(self.0.iter().map(|_| FlexItemDbg))
            .finish()
    }
}

impl<P: Paint> Draw<P> for FlexCol<'_, P> {
    fn draw(&self, buf: &mut P, area: Area)
    {
        if area.is_collapsed() || self.0.is_empty() {
            return;
        }

        let mut min   = 0usize;
        let mut basis = 0usize;

        for &it in self.0 {
            let p = it .proportions();

            min   += p.height.min() as usize;
            basis += calc_grow(p.height, area.height) as usize;
        }

        let flexy_len    = (area.height as usize).saturating_sub(min) as f64;
        let growth_scale = if basis == 0
            { 0. }
            else { f64::min(1., flexy_len / basis as f64) };

        let mut used = 0;
        let mut remainder = 0f64;

        for &it in &self.0[..self.0.len() - 1] {
            let p = it.proportions();
            let growth
                = calc_grow(p.height, area.height) as f64
                * growth_scale
                + remainder;
            remainder = growth.fract();

            let height = std::cmp::min(
                p.height.min() + growth.trunc() as u16,
                area.height - used,
            );

            it.draw(buf, Area {
                x: area.x,
                y: area.y + used,
                width: area.width,
                height,
            });

            used += height;
        }

        self.0[self.0.len() - 1].draw(buf, Area {
            x: area.x,
            y: area.y + used,
            width: area.width,
            height: area.height - used,
        });
    }
}

impl<P: Paint> Proportional for FlexCol<'_, P> {
    fn proportions(&self) -> Proportions
    {
        self.0.iter()
            .fold(Proportions::ZERO, |Proportions { width, height }, it|
        {
            let p = it.proportions();

            Proportions {
                width:  width.add(p.width),
                height: height.join(p.height),
            }
        })
    }
}


/// Horizontal flex container.
///
/// For more information on how the items are drawn, see the [Module-level
/// documentation](self).
#[derive(Clone)]
pub struct FlexRow<'a, P: Paint>(pub &'a [&'a dyn FlexItem<P>]);

impl<'a, P: Paint> std::fmt::Debug for FlexRow<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str("FlexRow ")?;
        f.debug_list()
            .entries(self.0.iter().map(|_| FlexItemDbg))
            .finish()
    }
}

impl<P: Paint> Draw<P> for FlexRow<'_, P> {
    fn draw(&self, buf: &mut P, area: Area)
    {
        if area.is_collapsed() || self.0.is_empty() {
            return;
        }

        let mut min   = 0usize;
        let mut basis = 0usize;

        for &it in self.0 {
            let p = it .proportions();

            min   += p.width.min() as usize;
            basis += calc_grow(p.width, area.width) as usize;
        }

        let flexy_len    = (area.width as usize).saturating_sub(min) as f64;
        let growth_scale = if basis == 0
            { 0. }
            else { f64::min(1., flexy_len / basis as f64) };

        let mut used = 0;
        let mut remainder = 0f64;

        for &it in &self.0[..self.0.len() - 1] {
            let p = it.proportions();
            let growth
                = calc_grow(p.width, area.width) as f64
                * growth_scale
                + remainder;
            remainder = growth.fract();

            let width = std::cmp::min(
                p.width.min() + growth.trunc() as u16,
                area.width - used,
            );

            it.draw(buf, Area {
                x: area.x + used,
                y: area.y,
                width,
                height: area.height,
            });

            used += width;
        }

        self.0[self.0.len() - 1].draw(buf, Area {
            x: area.x + used,
            y: area.y,
            width: area.width - used,
            height: area.height,
        });
    }
}

impl<P: Paint> Proportional for FlexRow<'_, P> {
    fn proportions(&self) -> Proportions
    {
        self.0.iter()
            .fold(Proportions::ZERO, |Proportions { width, height }, it|
        {
            let p = it.proportions();

            Proportions {
                width:  width.join(p.width),
                height: height.add(p.height),
            }
        })
    }
}


#[inline]
fn calc_grow(range: Range, max: u16) -> u16
{
    range.max()
        .map(|v| std::cmp::min(v, max))
        .unwrap_or(max)
        .saturating_sub(range.min())
}


struct FlexItemDbg;

impl std::fmt::Debug for FlexItemDbg
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str("FlexItem")
    }
}
