use std::cell::Cell;

use crate::{style::StyledString, Pos};
use crate::Dim;
use crate::paint::Paint;
use super::{
    Widget,
    InteractiveWidget,
};
use termion::event::{Event, Key};

use crate::Area;

type Transformer = fn(&str) -> StyledString;

/// Configuration options for theming [Menu].
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

/// Simple themable menu-like widget.
#[derive(Debug, Clone)]
pub struct Menu {
    pub theme: Theme,
    items: Vec<String>,
    active_idx: usize,
    // HACK: FIXME: this is state related purely to rendering.
    scroll: Cell<usize>,
}

impl Menu {
    /// Creates a new `Menu`.
    pub fn new(items: &[&str]) -> Self
    {
        Self {
            items: items.iter()
                .map(|it| it.to_string())
                .collect(),
            active_idx: 0,
            scroll: Cell::new(0),
            theme: Theme::default(),
        }
    }

    /// Gets a reference to the currently selected item.
    #[inline]
    pub fn selected(&self) -> &str
    {
        &self.items[self.active_idx]
    }

    /// Gets the index of the currently selected item.
    #[inline]
    pub fn selected_idx(&self) -> usize
    {
        self.active_idx
    }

    /// Adjusts the theme.
    #[inline]
    pub fn theme(mut self, theme: Theme) -> Self
    {
        self.theme = theme;

        self
    }

    /// Gets a reference to the items.
    #[inline]
    pub fn items(&self) -> &[String]
    {
        &self.items
    }

    #[inline]
    fn visible_count(&self, height: u16) -> u16
    {
        std::cmp::min(height as usize, self.items.len()) as u16
    }

    #[inline]
    fn active_item_location(&self, dimensions: Dim) -> Location
    {
        if self.active_idx < self.scroll.get() {
            Location::Above
        } else if self.active_idx < self.scroll.get() + dimensions.height as usize {
            Location::InView
        } else {
            Location::Below
        }
    }
}

impl<P: Paint> Widget<P> for Menu {
    fn render(&self, buf: &mut P, area: Area)
    {
        if area.is_void() {
            return;
        }

        match self.active_item_location(area.dimensions()) {
            Location::Above => self.scroll.set(self.active_idx),
            Location::InView => {},
            Location::Below => self.scroll.set(self.active_idx
                .saturating_sub(area.height as usize + 1)),
        }

        let start = self.scroll.get();
        let end = self.scroll.get() + self.visible_count(area.height) as usize;

        for (i, item) in self.items[start..end].iter().enumerate() {
            let item_i = start + i;

            let transform = if self.active_idx == item_i
                { self.theme.selected }
                else { self.theme.normal };
            let item = transform(item);
            buf.print(Pos{x:0, y:i as u16}, &item, area);
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
            // TODO: mouse support
            _ => (),
        }
    }
}
