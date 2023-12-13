use termion::event::Event;

pub mod bar;
pub mod inputline;
pub mod menu;
pub mod prompt;
pub mod frame;
pub mod canvas;

pub use bar::{HorizBar, VertBar};
pub use inputline::InputLine;
pub use menu::Menu;
pub use prompt::Prompt;
pub use frame::Frame;
pub use canvas::Canvas;

use crate::Area;
use crate::paint::Paint;

pub trait Widget {
    fn render(&self, buf: &mut impl Paint, area: Area);
}

pub trait InteractiveWidget {
    fn process_event(&mut self, e: Event);
}

/// A dummy widget that does nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dummy;

impl Widget for Dummy {
    fn render(&self, _buf: &mut impl Paint, _area: Area) {}
}

impl InteractiveWidget for Dummy {
    fn process_event(&mut self, _e: Event) {}
}
