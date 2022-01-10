use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use crate::pos;

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub z_index: u32,
    pub hidden: bool,
}

pub struct InnerWidget(Rc<RefCell<InnerWidgetBody>>);

impl InnerWidget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self (
            Rc::new(RefCell::new(
                InnerWidgetBody {
                    buffer: vec!['\0'; width * height],
                    start_y,
                    start_x,
                    height,
                    width,
                    z_index: 1,
                    hidden: true,
                }
            ))
        )
    }

    pub fn share(&self) -> Self
    {
        InnerWidget(Rc::clone(&self))
    }

    pub fn print(&mut self, y: u32, x: u32, line: &str)
    {
        let mut body = self.borrow_mut();

        if x as usize >= body.width || y as usize >= body.height {
            return;
        }

        let w = body.width;

        for (i, c) in line.chars().enumerate() {
            if x as usize + i >= body.width as usize {
                break;
            }
            body.buffer[pos![w, y as usize, x as usize + i]] = c;
        }
    }

    pub fn putc(&mut self, y: u32, x: u32, c: char)
    {
        let mut body = self.borrow_mut();

        if x as usize >= body.width || y as usize >= body.height {
            return;
        }

        let w = body.width;
        body.buffer[pos![w, y as usize, x as usize]] = c;
    }

    pub fn clear(&mut self)
    {
        for c in self.borrow_mut().buffer.iter_mut() {
            *c = '\0';
        }
    }
}

impl Deref for InnerWidget {
    type Target = Rc<RefCell<InnerWidgetBody>>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}