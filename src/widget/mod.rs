use termion::event::Event;

pub mod bar;
pub mod inputline;
pub mod menu;
pub mod prompt;
pub mod frame;

pub use bar::{HorizBar, VertBar};
pub use inputline::InputLine;
pub use menu::Menu;
pub use prompt::Prompt;
pub use frame::Frame;

use crate::{Area, screen::Buffer};

pub trait Widget {
    fn render(&mut self, buf: &mut Buffer, area: Area);
}

pub trait InteractiveWidget {
    fn process_event(&mut self, e: Event);
}

pub trait OutputtingWidget<T> : InteractiveWidget {
    fn get_output(&self) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullWidget;

impl Widget for NullWidget {
    fn render(&mut self, _buf: &mut Buffer, _area: Area) {}
}

impl InteractiveWidget for NullWidget {
    fn process_event(&mut self, _e: Event) {}
}

impl OutputtingWidget<()> for NullWidget {
    fn get_output(&self) -> Option<()> { None }
}
