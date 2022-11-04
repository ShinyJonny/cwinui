use termion::event::Event;

use crate::layout::{
    Aligned,
    Alignable,
    Align,
};
use crate::sub_impl_aligned;
use crate::sub_impl_alignable;
use crate::style::{StyledString, StyledStr, Style, StyledChar};

use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
    InputLine,
    PoisonError,
};

struct Theme {
    sep: StyledString,
    input_style: Style,
    input_blank_c: StyledChar,
}

pub struct Prompt {
    win: Window,
    label: StyledString,
    inputline: InputLine,
    theme: Theme,
}

impl Prompt {
    pub fn new<'s, T>(label: T, y: u32, x: u32, len: usize) -> Self
    where
        T: Into<StyledStr<'s>>
    {
        let label = label.into().to_owned();
        let sep = StyledString::from(": ");

        let label_len = label.content.chars().count();
        let sep_len = sep.content.chars().count();
        let prefix_len = label_len + sep_len;

        if prefix_len + 2 > len {
            panic!("prompt is not large enough");
        }

        let input_len = len - prefix_len;
        let input_x = x + prefix_len as u32;

        let mut inputline = InputLine::new(y, input_x, input_len);
        let win = Window::new(y, x, 1, len);

        inputline.show();
        win.share_inner().add_subwidget(inputline.share_inner());

        let mut prompt = Self {
            win,
            label,
            inputline,
            theme: Theme {
                sep,
                input_style: Style::default(),
                input_blank_c: ' '.into(),
            },
        };
        prompt.redraw();

        prompt
    }

    pub fn theme<'t, T, C>(
        mut self,
        sep: T,
        input_style: Style,
        input_blank_c: C
    ) -> Self
    where
        T: Into<StyledStr<'t>>,
        C: Into<StyledChar>
    {
        let input_blank_c = input_blank_c.into();
        let sep = sep.into();

        let prefix_len = self.label.content.chars().count() + sep.content.chars().count();
        if prefix_len + 2 > self.win.content_width() {
            panic!("prompt is not large enough");
        }

        let (start_y, start_x) = self.win.content_yx();
        self.inputline.change_pos(start_y, start_x + prefix_len as u32);
        self.inputline.resize(self.win.content_width() - prefix_len);

        self.theme = Theme {
            // FIXME: get rid of these allocations.
            sep: sep.to_owned(),
            input_style,
            input_blank_c
        };
        self.inputline.set_theme(self.theme.input_blank_c, self.theme.input_style);
        self.redraw();

        self
    }

    pub fn set_theme<'t, T, C>(
        &mut self,
        sep: T,
        input_style: Style,
        input_blank_c: C
    )
    where
        T: Into<StyledStr<'t>>,
        C: Into<StyledChar>
    {
        let input_blank_c = input_blank_c.into();
        let sep = sep.into();

        let prefix_len = self.label.content.chars().count() + sep.content.chars().count();
        if prefix_len + 2 > self.win.content_width() {
            panic!("prompt is not large enough");
        }

        let (start_y, start_x) = self.win.content_yx();
        self.inputline.change_pos(start_y, start_x + prefix_len as u32);
        self.inputline.resize(self.win.content_width() - prefix_len);

        self.theme = Theme {
            // FIXME: get rid of these allocations.
            sep: sep.to_owned(),
            input_style,
            input_blank_c,
        };
        self.inputline.set_theme(self.theme.input_blank_c, self.theme.input_style);
        self.redraw();
    }

    pub fn set_label<'t, T>(&mut self, label: T)
    where
        T: Into<StyledStr<'t>>
    {
        let label = label.into();

        let prefix_len = label.content.chars().count() + self.theme.sep.content.chars().count();
        if prefix_len + 2 > self.win.content_width() {
            panic!("prompt is not large enough");
        }

        let (start_y, start_x) = self.win.content_yx();
        self.inputline.change_pos(start_y, start_x + prefix_len as u32);
        self.inputline.resize(self.win.content_width() - prefix_len);

        // FIXME: get rid of these allocations.
        self.label = label.to_owned();
        self.redraw();
    }

    // TODO: is_active
    // TODO: set_active
    // TODO: set_inactive

    fn redraw(&mut self)
    {
        let sep_x = self.label.content.chars().count() as u32;

        self.win.print(0, 0, &self.label);
        self.win.print(0, sep_x, &self.theme.sep);
        self.inputline.redraw();
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
    fn try_get_output(&self) -> Option<String>
    {
        self.inputline.try_get_output()
    }

    fn get_output(&self) -> Result<String, PoisonError<String>>
    {
        self.inputline.get_output()
    }
}

sub_impl_aligned!(Prompt, win);
sub_impl_alignable!(Prompt, win, [inputline]);
