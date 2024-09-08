use std::{cell::RefCell, rc::Rc};

use gtk::cairo::Rectangle;

use crate::{axes::Axes, axis::Axis};

struct Plot {
    axes: Vec<(Rc<RefCell<Axes>>, f64)>,
}

impl Plot {
    pub fn draw(&self, cx: &gtk::cairo::Context, width: i32, height: i32) {
        let h_sum: f64 = self.axes.iter().map(|(_, h)| h).sum();

        let mut y = 0.0;
        for (ax, h) in self.axes.iter() {
            let row_height_px = (*h / h_sum * height as f64).round();
            ax.borrow_mut().draw(
                cx,
                gtk::cairo::Rectangle::new(0.0, y, width as f64, row_height_px),
            );
            y += row_height_px;
        }
    }
}

pub mod demo {
    use super::*;
    use gtk::{cairo::Rectangle, prelude::*};
    use std::{cell::RefCell, f64::consts::PI, rc::Rc};

    pub fn main() -> gtk::glib::ExitCode {
        let app = gtk::Application::builder().build();

        app.connect_activate(|app| {
            let plot_win = PlotWindow::new();
            plot_win.build_ui(app);
        });
        app.run()
    }

    struct PlotWindow {
        darea: gtk::DrawingArea,
    }

    impl PlotWindow {
        fn new() -> Self {
            // pass
            Self {}
        }

        fn add_axes(&mut self) {
            let axes = Rc::new(RefCell::new(Axes::new(Extents {
                xmin: -2.0 * PI,
                xmax: 2.0 * PI,
                ymin: -1.1,
                ymax: 1.1,
            })));
        }

        pub fn build_ui(&self, app: &gtk::Application) {
            let darea = gtk::DrawingArea::builder().content_height(500).content_width(800).build();

            let darea = Rc::new(RefCell::new(darea));
        }
    }
}
