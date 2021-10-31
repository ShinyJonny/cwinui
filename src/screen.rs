use std::cmp::Ordering;

pub struct Screen {
    buffer: Vec<Vec<char>>,
    height: u32,
    width: u32,
    widgets: Vec<Widget>
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

        for i in 0..self.widgets.len()
        {
            if self.widgets[i].has_border {
                self.draw_border(WidgetHandle { index: i });
            }

            self.draw_pane(WidgetHandle { index: i });
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
        self.widgets.push(Widget::new(start_y, start_x, height, width));
        WidgetHandle { index: self.widgets.len() - 1 }
    }

    // (horizontal bars, vertical bars, top-left corner, top-right corner, bottom-left corner,
    // bottom-right corner)
    pub fn w_set_border(&mut self, widget: &WidgetHandle, border: (char, char, char, char, char, char))
    {
        self.widgets[widget.index].border_style = border;
    }

    pub fn w_toggle_border(&mut self, widget: &WidgetHandle)
    {
        let has_border = &mut self.widgets[widget.index].has_border;

        if *has_border {
            *has_border = false;
        } else {
            *has_border = true;
        }
    }

    pub fn w_set_zindex(&mut self, widget: &WidgetHandle, z_index: u32)
    {
        self.widgets[widget.index].z_index = z_index;
    }

    fn draw_border(&mut self, w: WidgetHandle)
    {
        let width = self.widgets[w.index].width as usize;
        let height = self.widgets[w.index].height as usize;
        let start_y = self.widgets[w.index].start_y as usize;
        let start_x = self.widgets[w.index].start_x as usize;
        let border_chars = self.widgets[w.index].border_style;

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

    fn draw_pane(&mut self, w: WidgetHandle)
    {
        let mut width = self.widgets[w.index].width as usize;
        let mut height = self.widgets[w.index].height as usize;
        let mut start_x = self.widgets[w.index].start_x as usize;
        let mut start_y = self.widgets[w.index].start_y as usize;

        if self.widgets[w.index].has_border
        {
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
                let c = self.widgets[w.index].buffer[y][x];

                if c == '\0' {
                    continue;
                }

                self.buffer[start_y + y][start_x + x] = c;
            }
        }
    }
}


pub struct WidgetHandle {
    index: usize
}


#[derive(Eq, Ord)]
pub struct Widget {
    buffer: Vec<Vec<char>>,
    start_y: u32,
    start_x: u32,
    width: u32,
    height: u32,
    z_index: u32,
    has_border: bool,
    border_style: (char, char, char, char, char, char)
}

impl Widget {
    pub fn new(start_y: u32, start_x: u32, height: u32, width: u32) -> Self
    {
        Self {
            buffer: vec![vec!['\0'; width as usize]; height as usize],
            start_y,
            start_x,
            height,
            width,
            z_index: 1,
            has_border: false,
            border_style: (' ', ' ', ' ', ' ', ' ', ' '),
        }
    }
}

// Widgets are sorted based on their z_index.
// This simplifies the mechanisms for drawing or calculating which parts to draw.

impl PartialOrd for Widget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.z_index.cmp(&other.z_index))
    }
}

impl PartialEq for Widget {
    fn eq(&self, other: &Self) -> bool
    {
        self.z_index == other.z_index
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widgets_equal()
    {
        let mut a = Widget::new(0, 0, 0, 0);
        let mut b = Widget::new(0, 0, 0, 0);
        a.z_index = 1;
        b.z_index = 1;

        assert!(a == b);
    }

    #[test]
    fn widgets_not_equal()
    {
        let mut a = Widget::new(0, 0, 0, 0);
        let mut b = Widget::new(0, 0, 0, 0);
        a.z_index = 1;
        b.z_index = 2;

        assert!(a != b);
    }

    #[test]
    fn widgets_greater()
    {
        let mut a = Widget::new(0, 0, 0, 0);
        let mut b = Widget::new(0, 0, 0, 0);
        a.z_index = 2;
        b.z_index = 1;

        assert!(a > b);
    }

    #[test]
    fn widgets_smaller_or_eq()
    {
        let mut a = Widget::new(0, 0, 0, 0);
        let mut b = Widget::new(0, 0, 0, 0);
        a.z_index = 0;
        b.z_index = 1;

        assert!(a <= b);
    }

    #[test]
    fn widgets_sort()
    {
        let mut a = Widget::new(0, 0, 0, 0);
        let mut b = Widget::new(0, 0, 0, 0);
        let mut c = Widget::new(0, 0, 0, 0);
        let mut d = Widget::new(0, 0, 0, 0);
        let mut e = Widget::new(0, 0, 0, 0);
        let mut f = Widget::new(0, 0, 0, 0);
        let mut g = Widget::new(0, 0, 0, 0);
        a.z_index = 0;
        b.z_index = 1;
        c.z_index = 7;
        d.z_index = 3;
        e.z_index = 9;
        f.z_index = 4;
        g.z_index = 2;

        let mut vector = vec![a, b, c, d, e, f, g];
        vector.sort();

        let mut last_z = 0;

        for w in vector
        {
            assert!(last_z <= w.z_index);
            last_z = w.z_index;
        }
    }
}
