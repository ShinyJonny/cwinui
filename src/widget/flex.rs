use super::Widget;
use crate::paint::Paint;
use crate::layout::{Proportional, Proportions, Range};
use crate::Area;


pub trait FlexItem<P: Paint>: Widget<P> + Proportional {}

impl<P: Paint, T> FlexItem<P> for T
where
    T: Widget<P> + Proportional {}


pub struct FlexCol<'a, P: Paint>(pub &'a [&'a dyn FlexItem<P>]);

impl<P: Paint> Widget<P> for FlexCol<'_, P> {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() || self.0.len() == 0 {
            return;
        }

        let mut min   = 0usize;
        let mut basis = 0usize;

        for &it in self.0 {
            let p = it .proportions();

            min   += p.vert.min as usize;
            basis += calc_grow(p.vert, area.height) as usize;
        }

        let flexy_len    = (area.height as usize).saturating_sub(min) as f64;
        let growth_scale = if basis == 0
            { 0. }
            else { f64::min(1., flexy_len / basis as f64) };

        let mut used = 0;
        for &it in self.0 {
            let p = it.proportions();
            let growth
                = (calc_grow(p.vert, area.height) as f64 * growth_scale) as u16;
            let height = std::cmp::min(
                p.vert.min + growth,
                area.height - used,
            );

            it.render(buf, Area {
                x: area.x,
                y: area.y + used,
                width: area.width,
                height,
            });

            used += height;
        }
    }
}

impl<P: Paint> Proportional for FlexCol<'_, P> {
    fn proportions(&self) -> Proportions
    {
        self.0.iter()
            .fold(Proportions::ZERO, |Proportions { horiz, vert }, it|
        {
            let p = it.proportions();

            Proportions {
                horiz: horiz.union(p.horiz),
                vert:  vert.add(p.vert),
            }
        })
    }
}


pub struct FlexRow<'a, P: Paint>(pub &'a [&'a dyn FlexItem<P>]);

impl<P: Paint> Widget<P> for FlexRow<'_, P> {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() || self.0.len() == 0 {
            return;
        }

        let mut min   = 0usize;
        let mut basis = 0usize;

        for &it in self.0 {
            let p = it .proportions();

            min   += p.horiz.min as usize;
            basis += calc_grow(p.horiz, area.width) as usize;
        }

        let flexy_len    = (area.width as usize).saturating_sub(min) as f64;
        let growth_scale = if basis == 0
            { 0. }
            else { f64::min(1., flexy_len / basis as f64) };

        let mut used = 0;
        for &it in self.0 {
            let p = it.proportions();
            let growth
                = (calc_grow(p.horiz, area.width) as f64 * growth_scale) as u16;
            let width = std::cmp::min(
                p.horiz.min + growth,
                area.width - used,
            );

            it.render(buf, Area {
                x: area.x + used,
                y: area.y,
                width,
                height: area.height,
            });

            used += width;
        }
    }
}

impl<P: Paint> Proportional for FlexRow<'_, P> {
    fn proportions(&self) -> Proportions
    {
        self.0.iter()
            .fold(Proportions::ZERO, |Proportions { horiz, vert }, it|
        {
            let p = it.proportions();

            Proportions {
                horiz: horiz.add(p.horiz),
                vert:  vert.union(p.vert),
            }
        })
    }
}


#[inline]
fn calc_grow(range: Range, max: u16) -> u16
{
    range.max
        .map(|v| std::cmp::min(v, max))
        .unwrap_or(max)
        .saturating_sub(range.min)
}
