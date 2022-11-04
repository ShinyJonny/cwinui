use crate::style::StyledString;
use super::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
    PoisonError,
};
use termion::event::{Event, Key};

use crate::layout::{
    Aligned,
    Alignable,
    Align,
};
use crate::sub_impl_aligned;
use crate::sub_impl_alignable;

type Transformer = fn(&str) -> StyledString;

struct Theme {
    normal: Transformer,
    selected: Transformer,
}

pub struct Menu {
    win: Window,
    items: Vec<String>,
    output: Option<usize>,
    active_item: usize,
    scroll: usize,
    theme: Theme,
}

impl Menu {
    pub fn new(
        y: u32,
        x: u32,
        size: Option<(usize, usize)>,
        items: &[&str],
    ) -> Self
    {
        let items: Vec<_> = items.iter().map(|it| String::from(*it)).collect();

        let (height, width) = if let Some((height, width)) = size {
            (height, width)
        } else {
            let item_lens = items.iter().map(|it| it.len());
            let longest = item_lens.reduce(|longest, it_len| std::cmp::max(longest, it_len)).unwrap_or(0);
            (items.len() + 3, longest)
        };

        let mut menu = Self {
            win: Window::new(y, x, height, width),
            items,
            output: None,
            active_item: 0,
            scroll: 0,
            theme: Theme {
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
            },
        };
        menu.redraw();

        menu
    }

    pub fn set_theme(&mut self, normal: Transformer, selected: Transformer)
    {
        self.theme.normal = normal;
        self.theme.selected = selected;

        self.redraw();
    }

    fn redraw(&mut self)
    {
        self.win.clear();

        let first_item = self.scroll;

        let mut i = first_item;
        while i < self.visible_count() {
            let transform = if self.active_item == i {
                self.theme.selected
            } else {
                self.theme.normal
            };

            let win_index = (i - first_item) as u32;
            self.win.print(win_index, 0, &transform(&self.items[i]));
            i += 1;
        }
    }

    fn visible_count(&self) -> usize
    {
        std::cmp::min(self.win.content_height(), self.items.len())
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
    fn try_get_output(&self) -> Option<usize>
    {
        self.output
    }

    fn get_output(&self) -> Result<usize, PoisonError<usize>>
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
sub_impl_alignable!(Menu, win);
