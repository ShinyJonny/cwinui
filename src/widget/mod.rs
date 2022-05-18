use termion::event::Event;

mod inner;
mod window;
mod bar;
mod prompt;
mod menu;

use crate::misc::PoisonError;

pub use inner::{InnerWidget, InnerWidgetBody};
pub use window::Window;
pub use bar::{HorizBar, VertBar};
pub use prompt::Prompt;
pub use menu::{Menu, ListStyle};

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
    fn try_get_output(&mut self) -> Option<T>;
    fn get_output(self) -> Result<T, PoisonError<T>>;
}
