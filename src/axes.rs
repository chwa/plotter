use gtk::cairo::Context;

use crate::axis::Axis;
use crate::cairo_utils::PixelContext;
use crate::grid::Grid;

pub struct Margins {
    // TODO integers?
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Default for Margins {
    fn default() -> Self {
        Margins {
            left: 75.0,
            right: 20.0,
            top: 20.0,
            bottom: 75.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Extents {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl Extents {
    pub fn shift(&mut self, dx: f64, dy: f64) {
        self.xmin += dx;
        self.xmax += dx;
        self.ymin += dy;
        self.ymax += dy;
    }

    pub fn zoom_at(&mut self, pos_x: f64, pos_y: f64, scale: f64) {
        let (pos_x, pos_y) = self.unit_to_data(pos_x, pos_y);

        self.xmin = pos_x - (pos_x - self.xmin) * scale;
        self.xmax = pos_x - (pos_x - self.xmax) * scale;

        self.ymin = pos_y - (pos_y - self.ymin) * scale;
        self.ymax = pos_y - (pos_y - self.ymax) * scale;
    }

    pub fn unit_to_data(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.xmin + x * (self.xmax - self.xmin),
            self.ymin + y * (self.ymax - self.ymin),
        )
    }
}

pub struct Axes {
    primary_x: Axis,
    primary_y: Axis,
    grid: Grid,

    margins: Margins,
}

impl Axes {
    pub fn new(e: Extents) -> Self {
        Self {
            primary_x: Axis::linear1((e.xmin, e.xmax)),
            primary_y: Axis::vertical((e.ymin, e.ymax)),
            grid: Grid {},
            margins: Margins::default(),
        }
    }

    /// Draw to a Cairo context
    pub fn draw(
        &mut self,
        cx: &Context,
        // pixel coordinates for the full Axes area (including margins):
        rect: gtk::cairo::Rectangle,
    ) {
        let ll = (
            rect.x() + self.margins.left,
            rect.y() + rect.height() - self.margins.bottom,
        );
        let width = rect.width() - self.margins.left - self.margins.right;
        let height = rect.height() - self.margins.bottom - self.margins.top;
        self.primary_x.draw(cx, ll, width);
        self.primary_y.draw(cx, ll, height);

        self.grid.draw(
            cx,
            gtk::cairo::Rectangle::new(
                rect.x() + self.margins.left,
                rect.y() + self.margins.top,
                width,
                height,
            ),
            &self.primary_x,
            &self.primary_y,
        );

        cx.set_line_width(1.0);
        cx.set_source_rgb(0.0, 0.0, 0.0);
        PixelContext::new(cx).rectangle(ll.0, ll.1, width, -height);
        cx.stroke().unwrap();

        cx.set_line_width(1.0);
        cx.set_source_rgb(1.0, 0.0, 0.0);
        cx.rectangle(
            (rect.x() - 0.5).round() + 0.5,
            (rect.y() - 0.5).round() + 0.5,
            rect.width().round(),
            rect.height().round(),
        );
        cx.stroke().unwrap();
    }
}

pub mod demo {
    use super::*;
    use gtk::{cairo::Rectangle, prelude::*};
    use std::{cell::RefCell, rc::Rc};

    pub fn main() -> gtk::glib::ExitCode {
        let app = gtk::Application::builder().application_id("axis-demo").build();

        app.connect_activate(build_ui);
        app.run()
    }

    fn build_ui(app: &gtk::Application) {
        let darea = gtk::DrawingArea::builder().content_height(100).content_width(200).build();

        let darea = Rc::new(RefCell::new(darea));

        let position = gtk::cairo::Rectangle::new(0.05, 0.05, 0.9, 0.9);

        let axes = Rc::new(RefCell::new(Axes::new(Extents {
            xmin: -1.733,
            xmax: -1.658,
            ymin: 0.0,
            ymax: 1.0,
        })));

        darea.borrow().set_draw_func(move |_da, cx, width, height| {
            cx.set_source_rgb(1.0, 1.0, 1.0);
            cx.paint().unwrap();
            axes.borrow_mut().draw(
                cx,
                Rectangle::new(
                    position.x() * width as f64,
                    position.y() * height as f64,
                    position.width() * width as f64,
                    position.height() * height as f64,
                ),
            );
        });

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .child(&*darea.borrow())
            .build();

        window.present();
    }
}
