use std::cmp::Ordering;
use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Widget {
    fn share_inner(&self) -> InnerWidget;
    fn set_zindex(&mut self, index: u32);
    fn hide(&mut self);
    fn show(&mut self);
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

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub z_index: u32,
    pub hidden: bool,
}

// InnerWidgets are sorted based on their z_index.
// This simplifies the mechanisms for drawing or calculating which parts to draw.

impl PartialOrd for InnerWidget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.borrow().z_index.cmp(&other.borrow().z_index))
    }
}

impl Ord for InnerWidget {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.borrow().z_index.cmp(&other.borrow().z_index)
    }
}

impl PartialEq for InnerWidget {
    fn eq(&self, other: &Self) -> bool
    {
        self.borrow().z_index == other.borrow().z_index
    }
}

impl Eq for InnerWidget {}
