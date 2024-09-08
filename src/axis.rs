use crate::cairo_utils::{text_aligned, PixelContext, TextPos};
use crate::locator::{LinLocator, Locator};

#[derive(Clone, Copy)]
enum AxisDirection {
    Left,
    Right,
    Bottom,
    Top,
}

struct AxisPlacement {
    start_pos: (f64, f64),
    length: f64,
    direction: AxisDirection,
}

enum AxisType {
    Lin,
    Log,
}

pub struct Axis {
    direction: AxisDirection,
    axis_type: AxisType,
    pub range: (f64, f64),
    label: Option<String>,
    pub locator: Box<dyn Locator>,
}

impl Axis {
    pub fn linear1(range: (f64, f64)) -> Self {
        Self {
            direction: AxisDirection::Bottom,
            axis_type: AxisType::Lin,
            range,
            label: Some(String::from("Horizontal axis [units]")),
            locator: Box::new(LinLocator::new(0.025)),
        }
    }

    pub fn vertical(range: (f64, f64)) -> Self {
        Self {
            direction: AxisDirection::Left,
            axis_type: AxisType::Lin,
            range,
            label: Some(String::from("Vertical axis [units]")),
            locator: Box::new(LinLocator::new(0.025)),
        }
    }

    pub fn zoom_at(&mut self, x: f64, scale: f64) {
        let width = self.range.1 - self.range.0;
        let x_01 = (x - self.range.0) / width;
        let new_width = scale * width;

        self.range.0 = x - x_01 * new_width;
        self.range.1 = x + (1.0 - x_01) * new_width;
    }

    pub fn draw(&self, cx: &gtk::cairo::Context, start_pos: (f64, f64), length: f64) {
        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        cx.set_line_width(1.0);

        let (ticks_major, ticks_minor, decimals);
        match self.direction {
            AxisDirection::Left | AxisDirection::Right => {
                // PixelContext::new(cx).rel_line_to(0.0, -length);
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
            _ => {
                // PixelContext::new(cx).rel_line_to(length, 0.0);
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
        }

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        self.draw_ticks(cx, length, ticks_major, 8.0, true, decimals);

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        self.draw_ticks(cx, length, ticks_minor, 3.0, false, 0);

        if let Some(text) = &self.label {
            // PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
            match self.direction {
                AxisDirection::Left => {
                    // PixelContext::new(cx).move_to(start_pos.0 - 30.0, start_pos.1 - length / 2.0);
                    text_aligned(
                        cx,
                        (start_pos.0, start_pos.1 - length / 2.0),
                        &text,
                        TextPos::Left,
                        15.0,
                        50.0,
                        true,
                        true,
                    );
                }
                AxisDirection::Right => {}
                AxisDirection::Bottom => {
                    // PixelContext::new(cx).move_to(start_pos.0 - 30.0, start_pos.1 - length / 2.0);
                    text_aligned(
                        cx,
                        (start_pos.0 + length / 2.0, start_pos.1),
                        &text,
                        TextPos::Bottom,
                        15.0,
                        50.0,
                        false,
                        true,
                    );
                }
                AxisDirection::Top => {}
            }
        }
        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
    }

    pub fn draw_ticks(
        &self,
        cx: &gtk::cairo::Context,
        length: f64,
        ticks: Vec<f64>,
        tick_size: f64,
        with_labels: bool,
        decimals: usize,
    ) {
        // save start position

        let start_point = cx.current_point().unwrap();

        for t in ticks {
            let t_01 = (t - self.range.0) / (self.range.1 - self.range.0);

            // let (mut x_px, mut y_px) = (width * start_pos.0, height * start_pos.1);
            let text = format!("{:.prec$}", t, prec = decimals);

            cx.move_to(start_point.0, start_point.1);

            match self.direction {
                AxisDirection::Left => {
                    PixelContext::new(cx).rel_move_to(0.0, -t_01 * length);
                    PixelContext::new(cx).rel_line_to(-tick_size, 0.0);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 - tick_size, start_point.1 - t_01 * length),
                            &text,
                            TextPos::Left,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Right => {
                    PixelContext::new(cx).rel_move_to(0.0, -t_01 * length);
                    PixelContext::new(cx).rel_line_to(tick_size, 0.0);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + tick_size, start_point.1 - t_01 * length),
                            &text,
                            TextPos::Right,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Top => {
                    PixelContext::new(cx).rel_move_to(t_01 * length, 0.0);
                    PixelContext::new(cx).rel_line_to(0.0, -tick_size);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + t_01 * length, start_point.1 - tick_size),
                            &text,
                            TextPos::Top,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Bottom => {
                    PixelContext::new(cx).rel_move_to(t_01 * length, 0.0);
                    PixelContext::new(cx).rel_line_to(0.0, tick_size);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + t_01 * length, start_point.1 + tick_size),
                            &text,
                            TextPos::Bottom,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
            }
            // dbg!((
            //     t,
            //     cx.current_point().unwrap(),
            //     cx.has_current_point().unwrap()
            // ));
        }
        cx.stroke().unwrap();
    }
}

// pub mod demo {
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

//         let horiz = vec![
//             (-23.45, 7.892, 0.2),
//             (-0.087, -0.0135, 0.3),
//             (-23.45, 7.892, 0.4),
//             (-23.45, -23.192, 0.8),
//             (2223.45, 2224.45, 0.6),
//             (2223.45, 2224.45, 0.5),
//             (2223.45, 2224.45, 0.15),
//         ];

//         let axs: Vec<_> = horiz
//             .iter()
//             .enumerate()
//             .map(|(i, (start, stop, length))| Rc::new(RefCell::new(Axis::linear1((*start, *stop)))))
//             .collect();

//         let axs_vert: Vec<_> = horiz
//             .iter()
//             .enumerate()
//             .map(|(i, (start, stop, length))| {
//                 Rc::new(RefCell::new(Axis::vertical((*start, *stop))))
//             })
//             .collect();

//         darea.borrow().set_draw_func(move |_da, cx, width, height| {
//             for ax in &axs {
//                 ax.borrow_mut().draw(cx, (0.1, 0.1), 0.9);
//             }
//             for ax in &axs_vert {
//                 ax.borrow_mut().draw(cx, (0.1, 0.1), 0.9);
//             }
//         });

//         let window = gtk::ApplicationWindow::builder()
//             .application(app)
//             .title("My GTK App")
//             .child(&*darea.borrow())
//             .build();

//         window.present();
//     }
// }
