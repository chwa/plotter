use crate::{axis::Axis, cairo_utils::PixelContext};

pub struct Grid {}

impl Grid {
    pub fn draw(
        &self,
        cx: &gtk::cairo::Context,
        rect: gtk::cairo::Rectangle,
        primary_x: &Axis,
        primary_y: &Axis,
    ) {
        cx.set_line_width(1.0);

        // move to lower left corner
        PixelContext::new(cx).move_to(rect.x(), rect.y() + rect.height());

        // save start position
        let start_point = cx.current_point().unwrap();

        let (x_ticks_major, x_ticks_minor, _) =
            primary_x.locator.get_ticks(primary_x.range, Some(50.0 / rect.width()));

        let (y_ticks_major, y_ticks_minor, _) =
            primary_y.locator.get_ticks(primary_y.range, Some(50.0 / rect.height()));

        let x_range = primary_x.range;
        let y_range = primary_y.range;

        // minor
        cx.set_source_rgb(0.925, 0.925, 0.925);
        for t in x_ticks_minor {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = (t - x_range.0) / (x_range.1 - x_range.0);
            // let x_px = width * (self.position.x() + t_01 * self.position.width());
            PixelContext::new(cx).rel_move_to(t_01 * rect.width(), 0.0);
            PixelContext::new(cx).rel_line_to(0.0, -rect.height());
        }
        for t in y_ticks_minor {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = (t - y_range.0) / (y_range.1 - y_range.0);
            PixelContext::new(cx).rel_move_to(0.0, -t_01 * rect.height());
            PixelContext::new(cx).rel_line_to(rect.width(), 0.0);
        }
        cx.stroke().unwrap();

        // major
        cx.set_source_rgb(0.8, 0.8, 0.8);
        for t in x_ticks_major {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = (t - x_range.0) / (x_range.1 - x_range.0);
            PixelContext::new(cx).rel_move_to(t_01 * rect.width(), 0.0);
            PixelContext::new(cx).rel_line_to(0.0, -rect.height());
        }
        for t in y_ticks_major {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = (t - y_range.0) / (y_range.1 - y_range.0);
            PixelContext::new(cx).rel_move_to(0.0, -t_01 * rect.height());
            PixelContext::new(cx).rel_line_to(rect.width(), 0.0);
        }
        cx.stroke().unwrap();
    }
}

// pub mod demo {
//     use crate::{axis::Axis, locator::LinLocator};

//     use super::*;
//     use gtk::prelude::*;
//     use std::{cell::RefCell, rc::Rc};

//     pub fn main() -> gtk::glib::ExitCode {
//         // Create a new application
//         let app = gtk::Application::builder()
//             .application_id("axis-demo")
//             .build();

//         // Connect to "activate" signal of `app`
//         app.connect_activate(build_ui);

//         // Run the application
//         app.run()
//     }

//     fn build_ui(app: &gtk::Application) {
//         let darea = gtk::DrawingArea::builder()
//             .content_height(700)
//             .content_width(1200)
//             .build();

//         let darea = Rc::new(RefCell::new(darea));

//         let position = gtk::cairo::Rectangle::new(0.1, 0.1, 0.8, 0.7);

//         let x_range = (-12.345, 78.9);
//         let y_range = (765.2, 992.4);

//         let xaxis = Rc::new(RefCell::new(Axis::linear1(
//             position.y() + position.height(),
//             position.x(),
//             position.x() + position.width(),
//             x_range,
//         )));

//         let yaxis = Rc::new(RefCell::new(Axis::vertical(
//             position.x(),
//             position.y(),
//             position.y() + position.height(),
//             y_range,
//         )));

//         let grid = Rc::new(RefCell::new(Grid {
//             position,
//             x_range,
//             x_locator: Box::new(LinLocator::new(0.025)),
//             y_range,
//             y_locator: Box::new(LinLocator::new(0.025)),
//         }));

//         darea.borrow().set_draw_func(move |_da, cx, width, height| {
//             xaxis.borrow_mut().draw(cx, width as f64, height as f64);
//             yaxis.borrow_mut().draw(cx, width as f64, height as f64);
//             grid.borrow_mut().draw(cx, width as f64, height as f64);
//         });

//         let window = gtk::ApplicationWindow::builder()
//             .application(app)
//             .title("My GTK App")
//             .child(&*darea.borrow())
//             .build();

//         window.present();
//     }
// }
