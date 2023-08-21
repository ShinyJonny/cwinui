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

pub trait InteractiveWidget : Widget {
    fn process_event(&mut self, e: Event);
}

pub trait OutputWidget<T> : Widget {
    fn try_get_output(&self) -> Option<T>;
    fn get_output(&self) -> Result<T, PoisonError<T>>;
}

// TODO: FIXME: rename this.
pub struct PoisonError<T>(T);

impl<T> PoisonError<T> {
    pub fn new(i: T) -> Self
    {
        Self(i)
    }

    pub fn into_inner(self) -> T
    {
        self.0
    }

    pub fn get_ref(&self) -> &T
    {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T
    {
        &mut self.0
    }
}
