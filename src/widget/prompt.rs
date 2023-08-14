use termion::event::Event;

use crate::Pos;
use crate::layout::Area;
use crate::style::{StyledString, StyledStr, Style, StyledChar, WithStyle};

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
    pub fn new<'s, T>(pos: Pos, label: T, len: u16) -> Self
    where
        T: Into<StyledStr<'s>>
    {
        let label = label.into().to_owned();
        let sep = StyledString::from(": ");

        let label_len = label.content.chars().count();
        let sep_len = sep.content.chars().count();
        let prefix_len = label_len + sep_len;

        if prefix_len + 2 > len as usize {
            panic!("prompt is not large enough");
        }

        let input_x = pos.x + prefix_len as u16;
        let input_len = len - prefix_len as u16;

        let mut inputline
            = InputLine::new(Pos { x: input_x, y: pos.y }, input_len);
        let win = Window::new(Area {
            x: pos.x,
            y: pos.y,
            width: len as u16,
            height: 1
        });

        inputline.show();
        win.share_inner().add_subwidget(inputline.share_inner());

        let mut prompt = Self {
            win,
            label,
            inputline,
            theme: Theme {
                sep,
                input_style: Style::default(),
                input_blank_c: ' '.styled(),
            },
        };
        prompt.redraw();

        prompt
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

        let prefix_len = (self.label.content.chars().count()
            + sep.content.chars().count()) as u16;

        let content_area = self.win.content_area();

        if prefix_len + 2 > content_area.width {
            panic!("prompt is not large enough");
        }

        let Area { x: start_x, y: start_y, width: _, height: _ } = content_area;
        todo!("change the pos of the inputline to `start_x + prefix_len` and `start_y`"); // TODO
        self.inputline.resize(content_area.width - prefix_len);

        self.theme = Theme {
            // FIXME: get rid of these allocations.
            sep: sep.to_owned(),
            input_style,
            input_blank_c
        };
        self.inputline.set_theme(self.theme.input_blank_c, self.theme.input_style);
        self.redraw();
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
        self.set_theme(sep, input_style, input_blank_c);

        self
    }

    pub fn set_label<'t, T>(&mut self, label: T)
    where
        T: Into<StyledStr<'t>>
    {
        let label = label.into();

        let prefix_len = (label.content.chars().count()
            + self.theme.sep.content.chars().count()) as u16;

        let content_area = self.win.content_area();

        if prefix_len + 2 > content_area.width {
            panic!("prompt is not large enough");
        }

        let Area { x: start_x, y: start_y, width: _, height: _ } = content_area;
        todo!("change the pos of the inputline to `start_x + prefix_len` and `start_y`"); // TODO
        self.inputline.resize(content_area.width - prefix_len);

        // FIXME: get rid of these allocations.
        self.label = label.to_owned();
        self.redraw();
    }

    // TODO: is_active
    // TODO: set_active
    // TODO: set_inactive

    fn redraw(&mut self)
    {
        let sep_x = self.label.content.chars().count() as u16;

        self.win.print(0, 0, &self.label);
        self.win.print(sep_x, 0, &self.theme.sep);
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
