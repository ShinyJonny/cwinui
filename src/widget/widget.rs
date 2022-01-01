use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Widget {
    fn share_inner(&self) -> InnerWidget;
    fn set_zindex(&mut self, index: u32);
    fn hide(&mut self);
    fn show(&mut self);
}

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub z_index: u32,
    pub hidden: bool,
}

pub struct InnerWidget {
    w: Rc<RefCell<InnerWidgetBody>>,
}

impl InnerWidget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self {
            w: Rc::new(RefCell::new(
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
        }
    }

    pub fn share(&self) -> Self
    {
        InnerWidget { w: self.w.clone() }
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
        &self.w
    }
}
