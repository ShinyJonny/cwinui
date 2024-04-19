use termion::event::Event;

use crate::Pos;
use crate::layout::Area;
use crate::paint::Paint;
use crate::style::{StyledString, StyledStr, Style, StyledChar};

use super::{
    Widget,
    InteractiveWidget,
    InputLine,
};


/// Configuration options for theming [Prompt].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub sep: StyledString,
    pub input_style: Style,
    pub blank_c: StyledChar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThemeInternal {
    sep: StyledString,
}

/// Prompt-like wrapper for [InputLine].
#[derive(Debug, Clone)]
pub struct Prompt {
    pub label: StyledString,
    theme: ThemeInternal,
    inputline: InputLine,
}

impl Prompt {
    /// Creates a new `Prompt`.
    pub fn new<'s, T>(label: T) -> Self
    where
        T: Into<StyledStr<'s>>
    {
        Self {
            label: label.into().to_owned(),
            inputline: InputLine::new(),
            theme: ThemeInternal {
                sep: StyledString::from(": "),
            },
        }
    }

    /// Gets a reference to the contents of the input.
    #[inline]
    pub fn content(&self) -> &str
    {
        self.inputline.content()
    }

    /// Adjusts the theme.
    #[inline]
    pub fn theme<'t, S, C>(
        mut self,
        sep: S,
        input_style: Style,
        blank_c: C
    ) -> Self
    where
        S: Into<StyledStr<'t>>,
        C: Into<StyledChar>
    {
        self.theme = ThemeInternal {
            sep: StyledString::from(sep)
        };
        self.inputline.theme = super::inputline::Theme {
            input_style: input_style.into(),
            blank_c: blank_c.into(),
        };

        self
    }

    /// Sets the theme.
    #[inline]
    pub fn set_theme(&mut self, theme: Theme)
    {
        let Theme { sep, input_style, blank_c } = theme;

        self.theme = ThemeInternal { sep };
        self.inputline.theme = super::inputline::Theme {
            input_style,
            blank_c,
        };
    }

    /// Sets the prompt to its *active* state.
    #[inline]
    pub fn set_active(&mut self)
    {
        self.inputline.active = true;
    }

    /// Sets the prompt to its *inactive* state.
    #[inline]
    pub fn set_inactive(&mut self)
    {
        self.inputline.active = false;
    }
}

impl<P: Paint> Widget<P> for Prompt {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() {
            return;
        }

        // TODO: utf8 support.
        let label_len = self.label.content.len();
        // TODO: utf8 support.
        let sep_len = self.theme.sep.content.len();

        let (label_area, sep_and_input_area) = area.split_vert_at(
            std::cmp::min(
                label_len,
                area.width as usize
            ) as u16
        );
        let (sep_area, input_area) = sep_and_input_area.split_vert_at(
            std::cmp::min(
                sep_len,
                sep_and_input_area.width as usize
            ) as u16
        );

        buf.print(Pos::ZERO, &self.label, label_area);
        buf.print(Pos::ZERO, &self.theme.sep, sep_area);
        self.inputline.render(buf, input_area);
    }
}

impl InteractiveWidget for Prompt {
    fn process_event(&mut self, e: Event)
    {
        self.inputline.process_event(e);
    }
}
