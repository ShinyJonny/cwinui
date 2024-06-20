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
//! use cwinui::alloc::buffer::Buffer;
//! use cwinui::widget::{
//!     layout::{Container, Center},
//!     Row,
//!     Wireframe,
//! };
//! use cwinui::render::{Render, Draw};
//! use cwinui::layout::{Dim, Proportions};
//!
//! let ui = |renderer: &mut Buffer| {
//!     let area = renderer.area();
//!
//!     let dim = Dim {
//!         width: 20,
//!         height: area.height,
//!     };
//!
//!     Center(
//!         Container::new(Row(&[&Wireframe::new(), &Wireframe::new()]))
//!             .size(Proportions::fixed(dim))
//!     ).draw(renderer, area);
//! };
//! ```


use crate::layout::{Proportional, Proportions};
use crate::render::{Draw, Render};


/// Vertical split of widgets.
///
/// The paint area is split equally among the items. For more information see
/// the [Module-level documentation](self)
pub struct Col<'a, R: Render>(pub &'a [&'a dyn Draw<R>]);

impl<R: Render> Draw<R> for Col<'_, R> {
    fn draw(&self, buf: &mut R, area: crate::Area)
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

impl<R: Render> Proportional for Col<'_, R> {
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}


/// Horizontal split of widgets.
///
/// The paint area is split equally among the items. For more information see
/// the [Module-level documentation](self)
pub struct Row<'a, R: Render>(pub &'a [&'a dyn Draw<R>]);

impl<R: Render> Draw<R> for Row<'_, R> {
    fn draw(&self, buf: &mut R, area: crate::Area)
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

impl<R: Render> Proportional for Row<'_, R> {
    fn proportions(&self) -> Proportions
    {
        Proportions::flexible()
    }
}
