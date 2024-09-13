use std::cell::RefCell;
use std::rc::Rc;

use gtk::cairo::Context;

use crate::axis::{Axis, AxisPlacement, AxisType};
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
            bottom: 60.0,
        }
    }
}

pub struct Trace {
    pub values: Vec<(f64, f64)>,
    pub bbox: gtk::cairo::Rectangle,
    #[allow(dead_code)]
    pub name: String,
}

impl Trace {
    pub fn new(values: Vec<(f64, f64)>, name: &str) -> Self {
        let mut s = Self {
            values,
            bbox: gtk::cairo::Rectangle::new(0.0, 0.0, 1.0, 1.0),
            name: name.to_owned(),
        };
        s.update_bbox();
        s
    }

    fn update_bbox(&mut self) {
        self.bbox = if self.values.len() < 2 {
            gtk::cairo::Rectangle::new(0.0, 0.0, 1.0, 1.0)
        } else {
            let (xmin, xmax, ymin, ymax) = self.values.iter().fold(
                (
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                ),
                |(xmin, xmax, ymin, ymax), (x, y)| {
                    (xmin.min(*x), xmax.max(*x), ymin.min(*y), ymax.max(*y))
                },
            );
            gtk::cairo::Rectangle::new(xmin, ymin, xmax - xmin, ymax - ymin)
        }
    }

    pub fn nearest_point(
        &self,
        t: f64,
        y: f64,
        tradius: f64,
        yradius: f64,
    ) -> Option<(f64, f64, f64)> {
        // TODO: for log scale, transform values first before finding closest point
        // or change the x, y arguments to this function to be relative (window) coords?

        let segment_start = self
            .values
            .partition_point(|(x, _)| *x < t - tradius)
            .saturating_sub(2);
        let segment_end =
            (self.values.partition_point(|(x, _)| *x < t + tradius) + 2).min(self.values.len());

        let values_norm: Vec<_> = self.values[segment_start..segment_end]
            .iter()
            .map(|(t, y)| (t / tradius, y / yradius))
            .collect();

        // normalized query point
        let t_norm = t / tradius;
        let y_norm = y / yradius;

        let distances: Vec<_> = self.values[segment_start..segment_end]
            .windows(2)
            .map(|s| {
                // normalized start point and segment vector:
                let start_x = s[0].0 / tradius;
                let start_y = s[0].1 / yradius;
                let dx = (s[1].0 - s[0].0) / tradius;
                let dy = (s[1].1 - s[0].1) / yradius;

                // length squared of segment:
                let l2 = dx * dx + dy * dy;

                // normalized projection onto the segment
                // (value between 0 and 1 means the projection lies on the segment)
                let proj = ((t_norm - start_x) * dx + (y_norm - start_y) * dy) / l2;
                let proj = proj.clamp(0.0, 1.0);

                let nearest_x = start_x + proj * dx;
                let nearest_y = start_y + proj * dy;

                let dist_x = nearest_x - t_norm;
                let dist_y = nearest_y - y_norm;

                let distance = dist_x * dist_x + dist_y * dist_y;

                let nearest_x = nearest_x * tradius;
                let nearest_y = nearest_y * yradius;

                (distance, nearest_x, nearest_y)
            })
            .collect();

        distances
            .into_iter()
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .filter(|(d, _, _)| *d < 1.0)
    }
}

pub struct Axes {
    pub primary_x: Rc<RefCell<Axis>>,
    pub primary_y: Rc<RefCell<Axis>>,
    pub grid: Grid,

    pub margins: Margins,

    pub traces: Vec<Trace>,
    pub cursor: Option<(f64, f64)>,
}

#[derive(Clone, Copy, Debug)]
pub enum AxesCursorPosition {
    Chart(f64, f64),
    XAxis(f64),
    YAxis(f64),
    None,
}

impl Axes {
    pub fn new(primary_x: Rc<RefCell<Axis>>, primary_y: Rc<RefCell<Axis>>) -> Self {
        Self {
            primary_x,
            primary_y,
            grid: Grid {},
            margins: Margins::default(),
            traces: vec![],
            cursor: None,
        }
    }

    pub fn linear(shared_x: Option<Rc<RefCell<Axis>>>) -> Rc<RefCell<Self>> {
        let primary_x = match shared_x {
            Some(axis) => axis,
            None => Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Bottom,
                AxisType::Lin,
                (-1.0, 1.0),
            ))),
        };
        Rc::new(RefCell::new(Self {
            primary_x,
            primary_y: Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Left,
                AxisType::Lin,
                (0.0, 1.0),
            ))),
            grid: Grid {},
            margins: Margins::default(),
            traces: vec![],
            cursor: None,
        }))
    }

    pub fn semilogx() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            primary_x: Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Bottom,
                AxisType::Log,
                (0.1, 1.0),
            ))),
            primary_y: Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Left,
                AxisType::Lin,
                (0.0, 1.0),
            ))),
            grid: Grid {},
            margins: Margins::default(),
            traces: vec![],
            cursor: None,
        }))
    }

    pub fn semilogy(shared_x: Option<Rc<RefCell<Axis>>>) -> Rc<RefCell<Self>> {
        let primary_x = match shared_x {
            Some(axis) => axis,
            None => Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Bottom,
                AxisType::Lin,
                (-1.0, 1.0),
            ))),
        };
        Rc::new(RefCell::new(Self {
            primary_x,
            primary_y: Rc::new(RefCell::new(Axis::new(
                AxisPlacement::Left,
                AxisType::Log,
                (0.1, 1.0),
            ))),
            grid: Grid {},
            margins: Margins::default(),
            traces: vec![],
            cursor: None,
        }))
    }

    pub fn add_trace(&mut self, t: Trace) {
        self.traces.push(t);
    }

    pub fn cursor_position(
        &self,
        rect: gtk::cairo::Rectangle,
        x: f64,
        y: f64,
    ) -> AxesCursorPosition {
        let chart_width = rect.width() - self.margins.left - self.margins.right;
        let chart_height = rect.height() - self.margins.top - self.margins.bottom;
        let x_01 = (x - rect.x() - self.margins.left) / chart_width;
        let y_01 = (y - rect.y() - self.margins.top) / chart_height;

        if 0.0 <= x_01 && x_01 <= 1.0 && 0.0 <= y_01 && y_01 <= 1.0 {
            AxesCursorPosition::Chart(x_01, y_01)
        } else if x_01 < 0.0 && 0.0 <= y_01 && y_01 <= 1.0 {
            AxesCursorPosition::YAxis(y_01)
        } else if y_01 > 1.0 && 0.0 <= x_01 && x_01 <= 1.0 {
            AxesCursorPosition::XAxis(x_01)
        } else {
            AxesCursorPosition::None
        }
    }

    pub fn snap_cursor(&self, pos: AxesCursorPosition) -> Option<(f64, f64)> {
        match pos {
            AxesCursorPosition::Chart(x, y) => {
                let data_x = self.primary_x.borrow().axis_to_data(x);
                let data_y = self.primary_y.borrow().axis_to_data(1.0 - y);
                let xrange = self.primary_x.borrow().range;
                let yrange = self.primary_y.borrow().range;
                let result = self
                    .traces
                    .iter()
                    .map(|t| {
                        t.nearest_point(
                            data_x,
                            data_y,
                            (xrange.1 - xrange.0) / 20.0,
                            (yrange.1 - yrange.0) / 10.0,
                        )
                    })
                    .flatten()
                    .min_by(|a, b| a.0.total_cmp(&b.0))
                    .map(|(_, x, y)| (x, y));

                if let Some((resx, resy)) = result {
                    // println!("Snap {data_x:.3} {data_y:.3} to {resx:.3} {resy:.3}");
                }
                result
            }
            _ => None,
        }
    }

    pub fn zoom_fit(&mut self) {
        if self.traces.len() > 0 {
            let (xmin, xmax, ymin, ymax) = self.traces.iter().fold(
                (
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                ),
                |(xmin, xmax, ymin, ymax), tr| {
                    let r = tr.bbox;
                    (
                        xmin.min(r.x()),
                        xmax.max(r.x() + r.width()),
                        ymin.min(r.y()),
                        ymax.max(r.y() + r.height()),
                    )
                },
            );
            self.primary_x.borrow_mut().range = (xmin, xmax);
            self.primary_y.borrow_mut().range = (ymin, ymax);
        }
    }

    pub fn zoom_at(&mut self, position: AxesCursorPosition, scale: f64) {
        match position {
            AxesCursorPosition::Chart(x, y) => {
                self.primary_x.borrow_mut().zoom_at(x, scale);
                self.primary_y.borrow_mut().zoom_at(1.0 - y, scale);
            }
            AxesCursorPosition::XAxis(x) => {
                self.primary_x.borrow_mut().zoom_at(x, scale);
            }
            AxesCursorPosition::YAxis(y) => {
                self.primary_y.borrow_mut().zoom_at(1.0 - y, scale);
            }
            AxesCursorPosition::None => {}
        }
    }

    /// Draw to a Cairo context
    pub fn draw(
        &self,
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
        self.primary_x.borrow().draw(cx, ll, width);
        self.primary_y.borrow().draw(cx, ll, height);

        self.grid.draw(
            cx,
            gtk::cairo::Rectangle::new(
                rect.x() + self.margins.left,
                rect.y() + self.margins.top,
                width,
                height,
            ),
            &self.primary_x.borrow(),
            &self.primary_y.borrow(),
        );

        if false {
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

        // draw the traces
        cx.rectangle(ll.0, ll.1, width, -height);
        cx.clip();
        for (i, t) in self.traces.iter().enumerate() {
            if t.values.len() > 0 {
                cx.move_to(
                    self.margins.left + width * self.primary_x.borrow().data_to_axis(t.values[0].0),
                    rect.y()
                        + self.margins.top
                        + height * (1.0 - self.primary_y.borrow().data_to_axis(t.values[0].1)),
                );
            }
            for (x, y) in &t.values[1..] {
                cx.line_to(
                    self.margins.left + width * self.primary_x.borrow().data_to_axis(*x),
                    rect.y()
                        + self.margins.top
                        + height * (1.0 - self.primary_y.borrow().data_to_axis(*y)),
                );
            }

            cx.identity_matrix();
            cx.set_line_width(2.0);
            cx.set_source_rgb(1.0 - 0.1 * (i as f64), 0.6 + 0.2 * (i as f64), 0.0);
            cx.stroke().unwrap();
        }
        cx.reset_clip();

        cx.set_line_width(1.0);
        cx.set_source_rgb(0.0, 0.0, 0.0);

        if let Some((x, y)) = self.cursor {
            let px_x = self.margins.left + width * self.primary_x.borrow().data_to_axis(x);
            let px_y = rect.y()
                + self.margins.top
                + height * (1.0 - self.primary_y.borrow().data_to_axis(y));
            cx.move_to(px_x - 5.0, px_y);
            cx.rel_line_to(10.0, 0.0);
            cx.move_to(px_x, px_y - 5.0);
            cx.rel_line_to(0.0, 10.0);
            cx.stroke().unwrap()
        }

        // chart area outline
        cx.set_line_width(1.0);
        cx.set_source_rgb(0.0, 0.0, 0.0);
        PixelContext::new(cx).rectangle(ll.0, ll.1, width, -height);
        cx.stroke().unwrap();
    }
}
/*
pub mod demo {
    use super::*;
    use gtk::{cairo::Rectangle, prelude::*};
    use std::{cell::RefCell, f64::consts::PI, rc::Rc};

    pub fn main() -> gtk::glib::ExitCode {
        let app = gtk::Application::builder().build();

        app.connect_activate(build_ui);
        app.run()
    }

    fn example_signals(axes: &mut Axes) {
        let xs: Vec<_> = (5i32..=5000).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_sin: Vec<_> = xs.iter().map(|x| x.sin() + 1.2).collect();
        let signal_a: Vec<_> = xs
            .iter()
            .map(|x| ((0.5 * x).powf(2.0)).sin() / (0.5 + x.abs()) + 1.2)
            .collect();
        let signal_sinc: Vec<_> = xs
            .iter()
            .map(|x| if *x == 0.0 { 1.0 } else { x.sin() / x + 1.2 })
            .collect();

        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_sin).collect(),
            "Signal",
        ));
        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_a).collect(),
            "Signal",
        ));
        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_sinc).collect(),
            "Signal",
        ));
    }

    fn export_svg(axes: &mut Axes) {
        let svg = gtk::cairo::SvgSurface::new(800.0, 500.0, Some("abc.svg")).unwrap();
        let mut cx = gtk::cairo::Context::new(svg).unwrap();
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.paint().unwrap();
        axes.draw(&mut cx, Rectangle::new(0.0, 0.0, 800.0, 500.0));
    }

    fn build_ui(app: &gtk::Application) {
        let darea = Rc::new(RefCell::new(
            gtk::DrawingArea::builder()
                .content_height(500)
                .content_width(800)
                .build(),
        ));

        struct SharedState {
            axes: Axes,
            /// current rectangle for the Axes (pixel coords), updated on draw:
            current_rect: Rectangle,
            /// cursor position from last motion event
            cursor: AxesCursorPosition,
        }

        let state = Rc::new(RefCell::new(SharedState {
            axes: Axes::linear(None),
            current_rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cursor: AxesCursorPosition::None,
        }));

        let st = state.clone();
        let position = gtk::cairo::Rectangle::new(0.0, 0.0, 1.0, 1.0);
        darea.borrow().set_draw_func(move |_da, cx, width, height| {
            cx.set_source_rgb(1.0, 1.0, 1.0);
            cx.paint().unwrap();

            st.borrow_mut().current_rect = Rectangle::new(
                position.x() * width as f64,
                position.y() * height as f64,
                position.width() * width as f64,
                position.height() * height as f64,
            );

            let rect = st.borrow().current_rect;
            st.borrow_mut().axes.draw(cx, rect);
        });

        example_signals(&mut state.borrow_mut().axes);
        {
            state.borrow_mut().axes.zoom_fit();
            export_svg(&mut state.borrow_mut().axes);
        }

        // Key event controller
        let key = gtk::EventControllerKey::new();
        let da = darea.clone();
        let st = state.clone();
        key.connect_key_pressed(move |_, _, u, _| {
            if u == 41 {
                st.borrow_mut().axes.zoom_fit();
                da.borrow().queue_draw();
            }
            gtk::glib::Propagation::Stop
        });
        darea.borrow().add_controller(key);

        // Motion event controller
        let motion = gtk::EventControllerMotion::new();
        let st = state.clone();
        motion.connect_motion(move |_, x, y| {
            let cursor = st
                .borrow()
                .axes
                .cursor_position(st.borrow().current_rect, x, y);
            st.borrow_mut().cursor = cursor;
        });
        darea.borrow().add_controller(motion);

        // Scroll event controller
        let zoom = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        let da = darea.clone();
        let st = state.clone();
        zoom.connect_scroll(move |s, _, y| {
            if s.current_event()
                .unwrap()
                .modifier_state()
                .contains(gtk::gdk::ModifierType::SHIFT_MASK)
            {
                let scale = 1.0 + 0.1 * y.clamp(-1.0, 1.0);
                let cursor = st.borrow().cursor;
                st.borrow_mut().axes.zoom_at(cursor, scale);
                da.borrow().queue_draw();
            }
            gtk::glib::Propagation::Stop
        });
        darea.borrow().add_controller(zoom);

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .child(&*darea.borrow())
            .build();

        let key = gtk::EventControllerKey::new();
        let da = darea.clone();
        key.connect_key_pressed(move |s, _, _, _| {
            s.forward(&*da.borrow());
            gtk::glib::Propagation::Stop
        });

        window.add_controller(key);

        window.present();
    }
}
*/
