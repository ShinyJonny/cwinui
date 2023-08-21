use crate::{style::StyledString, screen::Buffer, Dim};
use super::{
    Widget,
    InteractiveWidget,
    OutputtingWidget,
};
use termion::event::{Event, Key};

use crate::Area;

type Transformer = fn(&str) -> StyledString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub normal: Transformer,
    pub selected: Transformer,
}

impl Default for Theme {
    fn default() -> Self
    {
        Self {
            normal: |item| {
                let mut line = StyledString::from("  ");
                line.content.push_str(item);
                line
            },
            selected: |item| {
                let mut line = StyledString::from("* ");
                line.content.push_str(item);
                line
            },
        }
    }
}

enum Location {
    Above,
    InView,
    Below,
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub theme: Theme,
    items: Vec<String>,
    output: Option<usize>,
    active_idx: usize,
    // FIXME: this is state related purely to rendering.
    scroll: usize,
}

impl Menu {
    pub fn new(items: &[&str]) -> Self
    {
        Self {
            items: items.iter()
                .map(|it| it.to_string())
                .collect(),
            output: None,
            active_idx: 0,
            scroll: 0,
            theme: Theme::default(),
        }
    }

    #[inline]
    fn visible_count(&self, height: u16) -> u16
    {
        std::cmp::min(height as usize, self.items.len()) as u16
    }

    #[inline]
    fn active_item_location(&self, dimensions: Dim) -> Location
    {
        if self.active_idx < self.scroll {
            Location::Above
        } else if self.active_idx < self.scroll + dimensions.height as usize {
            Location::InView
        } else {
            Location::Below
        }
    }
}

impl Widget for Menu {
    fn render(&mut self, buf: &mut Buffer, area: Area)
    {
        if area.width == 0 || area.height == 0 {
            return;
        }

        match self.active_item_location(area.dimensions()) {
            Location::Above => self.scroll = self.active_idx,
            Location::InView => {},
            Location::Below => self.scroll = self.active_idx + area.height as usize - 1,
        }

        let start = self.scroll;
        let end = self.scroll + self.visible_count(area.height) as usize;

        for (i, item) in self.items[start..end].iter().enumerate() {
            let item_i = start + i;

            let transform = if self.active_idx == item_i
                { self.theme.selected }
                else { self.theme.normal };
            buf.print(area.x, area.y + i as u16, &transform(item))
        }
    }
}

impl InteractiveWidget for Menu {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Up) => {
                if self.active_idx > 0 {
                    self.active_idx -= 1;
                }
            },
            Event::Key(Key::Down) => {
                if self.active_idx + 1 < self.items.len() {
                    self.active_idx += 1;
                }
            },
            Event::Key(Key::Char('\n')) |
            Event::Key(Key::Char(' ')) => {
                self.output = Some(self.active_idx);
            },
            Event::Key(Key::Esc) => {
                // FIXME: cleaner implementation of exiting the menu.
                self.output = Some(self.items.len() - 1);
            },
            // TODO: mouse support
            _ => (),
        }
    }
}

impl OutputtingWidget<usize> for Menu {
    fn get_output(&self) -> Option<usize>
    {
        self.output
    }
}
