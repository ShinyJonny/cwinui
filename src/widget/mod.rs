use termion::event::Event;

use crate::{Area, Draw, Render};


pub mod bar;
pub mod border;
pub mod layout;
pub mod flex;
pub mod split;
pub mod text;
mod filler;
mod backdrop;
mod debug;
mod alloc;

pub use split::{Row, Col};
pub use flex::{FlexCol, FlexRow};
pub use bar::{HorizBar, VertBar};
pub use border::Border;
pub use filler::Filler;
pub use backdrop::Backdrop;
pub use debug::Wireframe;
pub use alloc::*;


/// Interactive widgets that can process events.
pub trait InteractiveWidget {
    /// Processes an event.
    fn process_event(&mut self, e: Event);
}


/// A widget that does nothing.
#[derive(Debug, Clone, Copy)]
pub struct Void;

impl<R: Render> Draw<R> for Void {
    fn draw(&self, _buf: &mut R, _area: Area) {}
}

impl InteractiveWidget for Void {
    fn process_event(&mut self, _e: Event) {}
}

impl crate::layout::Proportional for Void {
    fn proportions(&self) -> crate::layout::Proportions
    {
        crate::layout::Proportions::flexible()
    }
}
