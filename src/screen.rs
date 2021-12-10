use std::cmp::Ordering;
use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Screen {
    buffer: Vec<Vec<char>>,
    height: u32,
    width: u32,
    widgets: Vec<WidgetHandle>,
}

impl Screen {
    pub fn new(rows: u32, cols: u32) -> Self
    {
        Self {
            buffer: vec![vec![' '; cols as usize]; rows as usize],
            height: rows as u32,
            width: cols as u32,
            widgets: Vec::new(),
        }
    }

    // Very inefficient atm.
    // Draws every single widget, even those that are not visible.
    pub fn draw(&mut self)
    {
        self.widgets.sort();

        for row in self.buffer.iter_mut()
        {
            for c in row
            {
                *c = ' ';
            }
        }

        for i in 0..self.widgets.len()
        {
            if self.widgets[i].borrow().has_border {
                self.draw_border(i);
            }

            self.draw_pane(i);
        }
    }

    pub fn refresh(&self)
    {
        for row in &self.buffer
        {
            for c in row
            {
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn add_widget(&mut self, start_y: u32, start_x: u32, height: u32, width: u32) -> WidgetHandle
    {
        let w = WidgetHandle::new(start_y, start_x, height, width);
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

        if border_chars.0 != '\0' {
            for i in 0..(width - 1)
            {
                self.buffer[start_y][start_x + i] = border_chars.0;
                self.buffer[start_y + height - 1][start_x + i] = border_chars.0;
            }
        }
        if border_chars.1 != '\0' {
            for i in 0..(height - 1)
            {
                self.buffer[start_y + i][start_x] = border_chars.1;
                self.buffer[start_y + i][start_x + width - 1] = border_chars.1;
            }
        }
        if border_chars.2 != '\0' {
            self.buffer[start_y][start_x] = border_chars.2;
        }
        if border_chars.3 != '\0' {
            self.buffer[start_y][start_x + width - 1] = border_chars.3;
        }
        if border_chars.4 != '\0' {
            self.buffer[start_y + height - 1][start_x + width - 1] = border_chars.4;
        }
        if border_chars.5 != '\0' {
            self.buffer[start_y + height - 1][start_x] = border_chars.5;
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

        for y in 0..height
        {
            for x in 0..width
            {
                let c = w.buffer[y][x];

                if c == '\0' {
                    continue;
                }

                self.buffer[start_y + y][start_x + x] = c;
            }
        }
    }
}

pub struct WidgetHandle {
    w: Rc<RefCell<Widget>>,
}

impl WidgetHandle {
    pub fn new(start_y: u32, start_x: u32, height: u32, width: u32) -> Self
    {
        Self {
            w: Rc::new(RefCell::new(
                Widget {
                    buffer: vec![vec!['\0'; width as usize]; height as usize],
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

        if x >= width || y >= height {
            return;
        }

        for (i, c) in line.chars().enumerate()
        {
            if x as usize + i >= width as usize {
                break;
            }

            w.buffer[y as usize][x as usize + i] = c;
        }
    }

    pub fn clear(&mut self)
    {
        for row in self.w.borrow_mut().buffer.iter_mut()
        {
            for c in row
            {
                *c = '\0';
            }
        }
    }
}

impl Deref for WidgetHandle {
    type Target = Rc<RefCell<Widget>>;

    fn deref(&self) -> &Self::Target
    {
        &self.w
    }
}

impl Clone for WidgetHandle {
    fn clone(&self) -> Self
    {
        WidgetHandle { w: self.w.clone() }
    }
}

pub struct Widget {
    buffer: Vec<Vec<char>>,
    start_y: u32,
    start_x: u32,
    pub width: u32,
    pub height: u32,
    z_index: u32,
    has_border: bool,
    border_style: (char, char, char, char, char, char),
}

// Widgets are sorted based on their z_index.
// This simplifies the mechanisms for drawing or calculating which parts to draw.

impl PartialOrd for WidgetHandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.w.borrow().z_index.cmp(&other.w.borrow().z_index))
    }
}

impl Ord for WidgetHandle {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.w.borrow().z_index.cmp(&other.w.borrow().z_index)
    }
}

impl PartialEq for WidgetHandle {
    fn eq(&self, other: &Self) -> bool
    {
        self.w.borrow().z_index == other.w.borrow().z_index
    }
}

impl Eq for WidgetHandle {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_handles_equal()
    {
        let a = WidgetHandle::new(0, 0, 0, 0);
        let b = WidgetHandle::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 1;

        assert!(a == b);
    }

    #[test]
    fn widget_handles_not_equal()
    {
        let a = WidgetHandle::new(0, 0, 0, 0);
        let b = WidgetHandle::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 2;

        assert!(a != b);
    }

    #[test]
    fn widget_handles_greater()
    {
        let a = WidgetHandle::new(0, 0, 0, 0);
        let b = WidgetHandle::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 2;
        b.borrow_mut().z_index = 1;

        assert!(a > b);
    }

    #[test]
    fn widget_handles_smaller_or_eq()
    {
        let a = WidgetHandle::new(0, 0, 0, 0);
        let b = WidgetHandle::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;

        assert!(a <= b);
    }

    #[test]
    fn widget_handles_sort()
    {
        let a = WidgetHandle::new(0, 0, 0, 0);
        let b = WidgetHandle::new(0, 0, 0, 0);
        let c = WidgetHandle::new(0, 0, 0, 0);
        let d = WidgetHandle::new(0, 0, 0, 0);
        let e = WidgetHandle::new(0, 0, 0, 0);
        let f = WidgetHandle::new(0, 0, 0, 0);
        let g = WidgetHandle::new(0, 0, 0, 0);
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

        for w in vector
        {
            assert!(last_z <= w.borrow().z_index);
            last_z = w.borrow_mut().z_index;
        }
    }
}
