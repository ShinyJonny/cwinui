mod widget;
mod window;
mod bar;

pub use widget::{Widget, InnerWidget, InnerWidgetBody};
pub use window::Window;
pub use bar::{HorizBar, VertBar};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_handles_equal()
    {
        let a = InnerWidget::new(0, 0, 0, 0);
        let b = InnerWidget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 1;

        assert!(a == b);
    }

    #[test]
    fn widget_handles_not_equal()
    {
        let a = InnerWidget::new(0, 0, 0, 0);
        let b = InnerWidget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 2;

        assert!(a != b);
    }

    #[test]
    fn widget_handles_greater()
    {
        let a = InnerWidget::new(0, 0, 0, 0);
        let b = InnerWidget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 2;
        b.borrow_mut().z_index = 1;

        assert!(a > b);
    }

    #[test]
    fn widget_handles_smaller_or_eq()
    {
        let a = InnerWidget::new(0, 0, 0, 0);
        let b = InnerWidget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;

        assert!(a <= b);
    }

    #[test]
    fn widget_handles_sort()
    {
        let a = InnerWidget::new(0, 0, 0, 0);
        let b = InnerWidget::new(0, 0, 0, 0);
        let c = InnerWidget::new(0, 0, 0, 0);
        let d = InnerWidget::new(0, 0, 0, 0);
        let e = InnerWidget::new(0, 0, 0, 0);
        let f = InnerWidget::new(0, 0, 0, 0);
        let g = InnerWidget::new(0, 0, 0, 0);
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
