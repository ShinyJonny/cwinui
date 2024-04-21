use termion::event::Event;

use crate::Pos;
use crate::layout::{Area, Proportional, Proportions};
use crate::widget::Paint;
use crate::style::{Style, StyledChar, StyledStr, StyledString, WithStyle};

use super::{
    Widget,
    InteractiveWidget,
    InputLine,
};


/// Configuration options for theming [`Prompt`].
#[derive(Debug, Clone)]
pub struct Theme {
    pub sep: StyledString,
    pub input_style: Style,
    pub blank_c: StyledChar,
}

impl Default for Theme {
    fn default() -> Self
    {
        Self {
            sep: StyledString::from(" "),
            input_style: Style::default(),
            blank_c: 'c'.styled(),
        }
    }
}

#[derive(Debug, Clone)]
struct ThemeInternal {
    sep: StyledString,
}

/// Prompt-like wrapper for [`InputLine`].
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
    pub fn theme(mut self, theme: Theme) -> Self
    {
        let Theme { sep, input_style, blank_c } = theme;

        self.theme = ThemeInternal { sep };
        self.inputline.theme = super::inputline::Theme {
            input_style,
            blank_c,
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
        if area.is_collapsed() {
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

impl Proportional for Prompt {
    /// Prompt requires the width to be at least the length of:
    /// - the label
    /// - the separator
    /// - 1 for the input line
    fn proportions(&self) -> Proportions
    {
        use crate::layout::Range;

        let min = (self.label.content.len()
            + self.theme.sep.content.len()
            + 1) as u16;

        Proportions {
            horiz: Range::from(min),
            vert: Range::fixed(1)
        }
    }
}

impl InteractiveWidget for Prompt {
    fn process_event(&mut self, e: Event)
    {
        self.inputline.process_event(e);
    }
}
