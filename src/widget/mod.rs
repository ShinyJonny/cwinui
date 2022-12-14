use termion::event::Event;

mod inner;
mod window;
mod bar;
mod inputline;
mod menu;
mod prompt;

pub use inner::{InnerWidget, InnerWidgetBody};
pub use window::Window;
pub use bar::{HorizBar, VertBar};
pub use inputline::InputLine;
pub use menu::Menu;
pub use prompt::Prompt;

pub trait Widget {
    fn share_inner(&self) -> InnerWidget;

    // TODO: These could be maybe moved directly to InnerWidget.

    fn set_zindex(&mut self, index: u32)
    {
        self.share_inner().borrow_mut().z_index = index;
    }

    fn hide(&mut self)
    {
        self.share_inner().borrow_mut().hidden = true;
    }

    fn show(&mut self)
    {
        self.share_inner().borrow_mut().hidden = false;
    }
}

pub trait InteractiveWidget : Widget {
    fn process_event(&mut self, e: Event);
}

pub trait OutputWidget<T> : Widget {
    fn try_get_output(&self) -> Option<T>;
    fn get_output(&self) -> Result<T, PoisonError<T>>;
}

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
