use termion::event::Event;

pub mod bar;
pub mod inputline;
pub mod menu;
pub mod prompt;

pub use bar::{HorizBar, VertBar};
pub use inputline::InputLine;
pub use menu::Menu;
pub use prompt::Prompt;

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
