use std::cmp::Ordering;
use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! pos {
    ( $width:expr, $y:expr, $x:expr ) => {
        $y * $width + $x
    }
}

pub struct Screen {
    buffer: Vec<char>,
    height: usize,
    width: usize,
    widgets: Vec<Widget>,
}

impl Screen {
    pub fn new(rows: usize, cols: usize) -> Self
    {
        Self {
            buffer: vec![' '; cols * rows],
            height: rows,
            width: cols,
            widgets: Vec::new(),
        }
    }

    // Very inefficient atm.
    // Draws every single widget, even those that are not visible.
    pub fn draw(&mut self)
    {
        self.widgets.sort();

        for c in self.buffer.iter_mut() {
            *c = ' ';
        }

        for i in 0..self.widgets.len() {
            if self.widgets[i].borrow().has_border {
                self.draw_border(i);
            }

            self.draw_pane(i);
        }
    }

    pub fn refresh(&self)
    {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.buffer[pos![self.width, y, x]]);
            }
            println!();
        }
    }

    pub fn add_widget(&mut self, start_y: u32, start_x: u32, height: usize, width: usize) -> Widget
    {
        let w = Widget::new(start_y, start_x, height, width);
        self.widgets.push(w.clone());
        w
    }

    fn draw_border(&mut self, w: usize)
    {
        let w = self.widgets[w].borrow();

        let width = w.width as usize;
        let height = w.height as usize;
        let start_y = w.start_y as usize;
        let start_x = w.start_x as usize;
        let border_chars = w.border_style;

        let sw = self.width;

        if border_chars.0 != '\0' {
            for i in 0..(width - 1) {
                self.buffer[pos![sw, start_y, start_x + i]] = border_chars.0;
                self.buffer[pos![sw, start_y + height - 1, start_x + i]] = border_chars.0;
            }
        }
        if border_chars.1 != '\0' {
            for i in 0..(height - 1) {
                self.buffer[pos![sw, start_y + i, start_x]] = border_chars.1;
                self.buffer[pos![sw, start_y + i, start_x + width - 1]] = border_chars.1;
            }
        }
        if border_chars.2 != '\0' {
            self.buffer[pos![sw, start_y, start_x]] = border_chars.2;
        }
        if border_chars.3 != '\0' {
            self.buffer[pos![sw, start_y, start_x + width - 1]] = border_chars.3;
        }
        if border_chars.4 != '\0' {
            self.buffer[pos![sw, start_y + height - 1, start_x + width - 1]] = border_chars.4;
        }
        if border_chars.5 != '\0' {
            self.buffer[pos![sw, start_y + height - 1, start_x]] = border_chars.5;
        }
    }

    fn draw_pane(&mut self, w: usize)
    {
        let w = self.widgets[w].borrow();

        let mut width = w.width as usize;
        let mut height = w.height as usize;
        let mut start_x = w.start_x as usize;
        let mut start_y = w.start_y as usize;

        if w.has_border {
            if width <= 2 || height <= 2 {
                return;
            }

            width -= 2;
            height -= 2;
            start_x += 1;
            start_y += 1;
        }

        let ww = w.width;
        let sw = self.width;

        for y in 0..height {
            for x in 0..width {
                let c = w.buffer[pos![ww, y, x]];

                if c == '\0' {
                    continue;
                }

                self.buffer[pos![sw, start_y + y, start_x + x]] = c;
            }
        }
    }
}

pub struct Widget {
    w: Rc<RefCell<InnerWidget>>,
}

impl Widget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self {
            w: Rc::new(RefCell::new(
                InnerWidget {
                    buffer: vec!['\0'; width * height],
                    start_y,
                    start_x,
                    height,
                    width,
                    z_index: 1,
                    has_border: false,
                    border_style: (' ', ' ', ' ', ' ', ' ', ' '),
                }
            ))
        }
    }

    // (horizontal bars, vertical bars, top-left corner, top-right corner, bottom-left corner,
    // bottom-right corner)
    pub fn set_border(&mut self, border: (char, char, char, char, char, char))
    {
        self.w.borrow_mut().border_style = border;
    }

    pub fn toggle_border(&mut self)
    {
        let mut w = self.w.borrow_mut();

        if w.has_border {
            w.has_border = false;
        } else {
            w.has_border = true;
        }
    }

    pub fn set_zindex(&mut self, z_index: u32)
    {
        self.w.borrow_mut().z_index = z_index;
    }

    pub fn print(&mut self, mut y: u32, mut x: u32, line: &str)
    {
        let mut w = self.w.borrow_mut();

        let mut width = w.width;
        let mut height = w.height;

        if w.has_border {
            y += 1;
            x += 1;
            width -= 1;
            height -= 1;
        }

        if width < 1 || height < 1 {
            return;
        }

        if x as usize >= width || y as usize >= height {
            return;
        }

        let ww = w.width;

        for (i, c) in line.chars().enumerate() {
            if x as usize + i >= width as usize {
                break;
            }

            w.buffer[pos![ww, y as usize, x as usize + i]] = c;
        }
    }

    pub fn clear(&mut self)
    {
        for c in self.w.borrow_mut().buffer.iter_mut() {
            *c = '\0';
        }
    }
}

impl Deref for Widget {
    type Target = Rc<RefCell<InnerWidget>>;

    fn deref(&self) -> &Self::Target
    {
        &self.w
    }
}

impl Clone for Widget {
    fn clone(&self) -> Self
    {
        Widget { w: self.w.clone() }
    }
}

pub struct InnerWidget {
    buffer: Vec<char>,
    start_y: u32,
    start_x: u32,
    pub width: usize,
    pub height: usize,
    z_index: u32,
    has_border: bool,
    border_style: (char, char, char, char, char, char),
}

// Widgets are sorted based on their z_index.
// This simplifies the mechanisms for drawing or calculating which parts to draw.

impl PartialOrd for Widget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.w.borrow().z_index.cmp(&other.w.borrow().z_index))
    }
}

impl Ord for Widget {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.w.borrow().z_index.cmp(&other.w.borrow().z_index)
    }
}

impl PartialEq for Widget {
    fn eq(&self, other: &Self) -> bool
    {
        self.w.borrow().z_index == other.w.borrow().z_index
    }
}

impl Eq for Widget {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_handles_equal()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 1;

        assert!(a == b);
    }

    #[test]
    fn widget_handles_not_equal()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 2;

        assert!(a != b);
    }

    #[test]
    fn widget_handles_greater()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 2;
        b.borrow_mut().z_index = 1;

        assert!(a > b);
    }

    #[test]
    fn widget_handles_smaller_or_eq()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;

        assert!(a <= b);
    }

    #[test]
    fn widget_handles_sort()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        let c = Widget::new(0, 0, 0, 0);
        let d = Widget::new(0, 0, 0, 0);
        let e = Widget::new(0, 0, 0, 0);
        let f = Widget::new(0, 0, 0, 0);
        let g = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;
        c.borrow_mut().z_index = 7;
        d.borrow_mut().z_index = 3;
        e.borrow_mut().z_index = 9;
        f.borrow_mut().z_index = 4;
        g.borrow_mut().z_index = 2;

        let mut vector = vec![a, b, c, d, e, f, g];
        vector.sort();

        let mut last_z = 0;

        for w in vector {
            assert!(last_z <= w.borrow().z_index);
            last_z = w.borrow_mut().z_index;
        }
    }
}
