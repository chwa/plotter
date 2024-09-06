use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

struct Canvas {
    darea: gtk::DrawingArea,
    cursor_pos: Option<(f64, f64)>,
}

impl Canvas {
    fn new(width: i32, height: i32) -> Rc<RefCell<Self>> {
        let darea = gtk::DrawingArea::builder()
            .content_width(width)
            .content_height(height)
            .build();

        let c = Rc::new(RefCell::new(Self {
            darea,
            cursor_pos: None,
        }));

        let motion = gtk::EventControllerMotion::new();
        let c_clone = c.clone();
        motion.connect_motion(move |_, x, y| {
            c_clone.borrow_mut().cursor_pos = Some((x, y));
        });
        let c_clone = c.clone();
        motion.connect_leave(move |_| {
            c_clone.borrow_mut().cursor_pos = None;
        });

        c.borrow().darea.add_controller(motion);
        c
    }

    fn widget(&self) -> &gtk::DrawingArea {
        &self.darea
    }
}
