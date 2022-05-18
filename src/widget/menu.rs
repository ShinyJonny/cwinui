use crate::widget::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
};
use termion::event::{Event, Key};

use crate::layout::{Aligned, Align};
use crate::misc::PoisonError;
use crate::sub_impl_aligned;

pub enum ListStyle {
    Dummy,
}

pub struct Menu {
    win: Window,
    items: Vec<String>,
    output: Option<usize>,
    active_item: usize,
    scroll: usize,
    style: ListStyle,
}

impl Menu {
    pub fn new(
        y: u32,
        x: u32,
        height: usize,
        width: usize,
        items: &[&str],
        style: ListStyle
    ) -> Self
    {
        let mut new_items = Vec::with_capacity(items.len());
        for s in items {
            new_items.push(String::from(*s));
        }

        let mut ret = Self {
            win: Window::new(y, x, height, width),
            items: new_items,
            output: None,
            active_item: 0,
            scroll: 0,
            style,
        };
        ret.redraw();

        ret
    }

    fn redraw(&mut self)
    {
        self.win.clear();

        let start = self.scroll;
        let end = usize::min(self.scroll + self.win.content_height(), self.items.len());

        for (i, item) in self.items[start..end].iter().enumerate() {
            self.win.print(i as u32, 2, item);
        }
        if self.active_item >= start && self.active_item < end {
            self.win.putc((self.active_item - self.scroll) as u32, 0, '*');
        }
    }

    fn visible_count(&self) -> usize
    {
        self.win.content_height()
    }
}

impl Widget for Menu {
    fn share_inner(&self) -> InnerWidget
    {
        self.win.share_inner()
    }
}

impl InteractiveWidget for Menu {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Up) => {
                if self.active_item > 0 {
                    self.active_item -= 1;
                    if self.scroll > self.active_item {
                        self.scroll -= 1;
                    }
                    self.redraw();
                }
            },
            Event::Key(Key::Down) => {
                if self.active_item + 1 < self.items.len() {
                    self.active_item += 1;
                    if self.scroll + self.visible_count() < self.active_item + 1 {
                        self.scroll += 1;
                    }
                    self.redraw();
                }
            },
            Event::Key(Key::Char('\n')) |
            Event::Key(Key::Char(' ')) => {
                self.output = Some(self.active_item);
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

impl OutputWidget<usize> for Menu {
    fn try_get_output(&mut self) -> Option<usize>
    {
        self.output
    }

    fn get_output(self) -> Result<usize, PoisonError<usize>>
    {
        if let Some(o) = self.output {
            Ok(o)
        } else {
            // FIXME: is this really the correct way to do this???
            Err(PoisonError::new(0))
        }
    }
}

sub_impl_aligned!(Menu, win);
