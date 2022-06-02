use termion::event::Event;

use crate::layout::{
    Aligned,
    Alignable,
    Align,
};
use crate::sub_impl_aligned;
use crate::sub_impl_alignable;
use crate::misc::PoisonError;

use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
    InputLine,
};

pub struct Prompt {
    win: Window,
    label: String,
    inputline: InputLine,
}

impl Prompt {
    pub fn new(label: &str, y: u32, x: u32, len: usize) -> Self
    {
        let label = String::from(label);

        if len <= label.len() {
            panic!("length of Prompt is smaller or equal to the length o the label");
        }

        let input_len = len - label.len();
        let input_x = x + label.len() as u32;

        let mut inputline = InputLine::new(y, input_x, input_len);
        let win = Window::new(y, x, 1, len);

        inputline.show();
        win.share_inner().add_subwidget(inputline.share_inner());

        let mut prompt = Self {
            win,
            label,
            inputline,
        };
        prompt.redraw();

        prompt
    }

    pub fn is_active(&self) -> bool
    {
        self.inputline.is_active()
    }

    pub fn set_active(&mut self)
    {
        self.inputline.set_active();
    }

    pub fn set_inactive(&mut self)
    {
        self.inputline.set_inactive();
    }

    fn redraw(&mut self)
    {
        self.win.print(0, 0, &self.label);
        // TODO: redraw subwidgets.
    }
}

impl Widget for Prompt {
    fn share_inner(&self) -> InnerWidget
    {
        self.win.share_inner()
    }
}

impl InteractiveWidget for Prompt {
    fn process_event(&mut self, e: Event)
    {
        self.inputline.process_event(e);
    }
}

impl OutputWidget<String> for Prompt {
    fn try_get_output(&mut self) -> Option<String>
    {
        self.inputline.try_get_output()
    }

    fn get_output(self) -> Result<String, PoisonError<String>>
    {
        self.inputline.get_output()
    }
}

sub_impl_aligned!(Prompt, win);
sub_impl_alignable!(Prompt, win, [inputline]);
