use termion::event::Event;

use crate::layout::Area;
use crate::screen::Buffer;
use crate::style::{StyledString, StyledStr, Style, StyledChar, WithStyle};

use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InputLine,
    PoisonError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub sep: StyledString,
    pub input_style: Style,
    pub input_blank_c: StyledChar,
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub label: StyledString,
    pub theme: Theme,
    inputline: InputLine,
}

impl Prompt {
    pub fn new<'s, T>(label: T) -> Self
    where
        T: Into<StyledStr<'s>>
    {
        Self {
            label: label.into().to_owned(),
            inputline: InputLine::new(),
            theme: Theme {
                sep: StyledString::from(": "),
                input_style: Style::default(),
                input_blank_c: ' '.styled(),
            },
        }
    }

    pub fn theme<'t, S, C>(
        mut self,
        sep: S,
        input_style: Style,
        input_blank_c: C
    ) -> Self
    where
        S: Into<StyledStr<'t>>,
        C: Into<StyledChar>
    {
        self.theme = Theme {
            sep: StyledString::from(sep),
            input_style: input_style.into(),
            input_blank_c: input_blank_c.into(),
        };

        self
    }

    // TODO: is_active
    // TODO: set_active
    // TODO: set_inactive
}

impl Widget for Prompt {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        // TODO: utf8 support.
        let sep_len = self.theme.sep.content.len();
        // TODO: utf8 support.
        let label_len = self.label.content.len();
        let prefix_len = sep_len + label_len;

        let input_len = if prefix_len >= area.width as usize
            { 1 }
            else { area.width as usize - prefix_len };
        let visible_prefix_len = area.width as usize - input_len;
        let (_, input_area) = area.split_horiz_at(visible_prefix_len as u16);

        self.inputline.render(buf, input_area);

        // TODO: implement direct slicing of `StyledStr`.
        // TODO: utf8 support.
        let to_print_lab = self.label.content[
            ..std::cmp::min(label_len, visible_prefix_len)
        ].with_style(|_| self.label.style);
        buf.print(area.x, area.y, to_print_lab);

        // TODO: utf8 support.
        let to_print_sep = self.theme.sep.content[
            ..visible_prefix_len.saturating_sub(label_len)
        ].with_style(|_| self.theme.sep.style);
        if to_print_sep.content.len() > 0 {
            buf.print(area.x + label_len as u16, area.y, to_print_sep);
        }
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
