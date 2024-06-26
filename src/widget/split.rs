//! Lists of widgets, rendered in equally-sized cells.
//!
//! Note that these widgets have fully flexible proportions
//! [`Proportional`](crate::layout::Proportional) as they ignore their items'
//! proportions anyway. To control the size of the splits, wrap them in a
//! [`Container`](super::layout::Container).
//!
//! To ensure all items have the same size, make the proportions of the
//! [`Container`](super::layout::Container) divisible by the number of items.
//! Otherwise, the last element will take its portion of space plus the
//! remainder.
//!
//! # Example
//!
//! ```
//! use cwinui::screen::RenderContext;
//! use cwinui::widget::{
//!     layout::{Container, Center},
//!     Row,
//!     Wireframe,
//! };
//! use cwinui::layout::{Dim, Proportions};
//!
//! let ui = |ctx: &mut RenderContext| {
//!     let area = ctx.area();
//!
//!     let dim = Dim {
//!         width: 20,
//!         height: area.height,
//!     };
//!
//!     ctx.draw_fullscreen(
//!         &Center(
//!             Container::new(Row(&[&Wireframe::new(), &Wireframe::new()]))
//!                 .size(Proportions::fixed(dim))
//!         )
//!     );
//! };
//! ```


use crate::layout::{Proportional, Proportions};
use super::{Draw, Paint};


/// Vertical split of widgets.
///
/// The paint area is split equally among the items. For more information see
/// the [Module-level documentation](self)
pub struct Col<'a, P: Paint>(pub &'a [&'a dyn Draw<P>]);

impl<P: Paint> Draw<P> for Col<'_, P> {
    fn draw(&self, buf: &mut P, area: crate::Area)
    {
        if area.is_collapsed() || self.0.is_empty() {
            return;
        }

        let window = (area.height as usize / self.0.len()) as u16;
        let mut remaining = area;

        let last_idx = self.0.len() - 1;
        for &w in &self.0[..last_idx] {
            let (cur_area, rest) = remaining.split_horiz_at(window);
            remaining = rest;

            w.draw(buf, cur_area);
        }

        self.0[last_idx].draw(buf, remaining);
    }
}

impl<P: Paint> Proportional for Col<'_, P> {
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}


/// Horizontal split of widgets.
///
/// The paint area is split equally among the items. For more information see
/// the [Module-level documentation](self)
pub struct Row<'a, P: Paint>(pub &'a [&'a dyn Draw<P>]);

impl<P: Paint> Draw<P> for Row<'_, P> {
    fn draw(&self, buf: &mut P, area: crate::Area)
    {
        if area.is_collapsed() || self.0.is_empty() {
            return;
        }

        let window = (area.width as usize / self.0.len()) as u16;
        let mut remaining = area;

        let last_idx = self.0.len() - 1;
        for &w in &self.0[..last_idx] {
            let (cur_area, rest) = remaining.split_vert_at(window);
            remaining = rest;

            w.draw(buf, cur_area);
        }

        self.0[last_idx].draw(buf, remaining);
    }
}

impl<P: Paint> Proportional for Row<'_, P> {
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}
