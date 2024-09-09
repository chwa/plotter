use gtk::cairo::{Context, Matrix};

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

pub struct Trace {
    pub values: Vec<(f64, f64)>,
    pub bbox: gtk::cairo::Rectangle,
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
}

pub struct Axes {
    primary_x: Axis,
    primary_y: Axis,
    grid: Grid,

    margins: Margins,

    traces: Vec<Trace>,
}

#[derive(Clone, Copy, Debug)]
enum CursorPosition {
    Chart(f64, f64),
    XAxis(f64),
    YAxis(f64),
    None,
}

impl Axes {
    pub fn new(e: Extents) -> Self {
        Self {
            primary_x: Axis::linear1((e.xmin, e.xmax)),
            primary_y: Axis::vertical((e.ymin, e.ymax)),
            grid: Grid {},
            margins: Margins::default(),
            traces: vec![],
        }
    }

    pub fn add_trace(&mut self, t: Trace) {
        self.traces.push(t);
    }

    pub fn cursor_position(&self, rect: gtk::cairo::Rectangle, x: f64, y: f64) -> CursorPosition {
        let chart_width = rect.width() - self.margins.left - self.margins.right;
        let chart_height = rect.height() - self.margins.top - self.margins.bottom;
        let x_01 = (x - rect.x() - self.margins.left) / chart_width;
        let y_01 = (y - rect.y() - self.margins.top) / chart_height;

        let data_x = self.primary_x.axis_to_data(x_01);
        let data_y = self.primary_y.axis_to_data(1.0 - y_01);

        if 0.0 <= x_01 && x_01 <= 1.0 && 0.0 <= y_01 && y_01 <= 1.0 {
            CursorPosition::Chart(data_x, data_y)
        } else if x_01 < 0.0 && 0.0 <= y_01 && y_01 <= 1.0 {
            CursorPosition::YAxis(data_y)
        } else if y_01 > 1.0 && 0.0 <= x_01 && x_01 <= 1.0 {
            CursorPosition::XAxis(data_x)
        } else {
            CursorPosition::None
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
            self.primary_x.range = (xmin, xmax);
            self.primary_y.range = (ymin, ymax);
        }
    }

    pub fn zoom_at(&mut self, position: CursorPosition, scale: f64) {
        match position {
            CursorPosition::Chart(x, y) => {
                self.primary_x.zoom_at(x, scale);
                self.primary_y.zoom_at(y, scale);
            }
            CursorPosition::XAxis(x) => {
                self.primary_x.zoom_at(x, scale);
            }
            CursorPosition::YAxis(y) => {
                self.primary_y.zoom_at(y, scale);
            }
            CursorPosition::None => {}
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
                    self.margins.left + width * self.primary_x.data_to_axis(t.values[0].0),
                    self.margins.top + height * (1.0 - self.primary_y.data_to_axis(t.values[0].1)),
                );
            }
            for (x, y) in &t.values[1..] {
                cx.line_to(
                    self.margins.left + width * self.primary_x.data_to_axis(*x),
                    self.margins.top + height * (1.0 - self.primary_y.data_to_axis(*y)),
                );
            }

            cx.identity_matrix();
            cx.set_line_width(2.0);
            cx.set_source_rgb(1.0 - 0.1 * (i as f64), 0.6 + 0.2 * (i as f64), 0.0);
            cx.stroke().unwrap();
        }
        cx.reset_clip();

        // chart area outline
        cx.set_line_width(1.0);
        cx.set_source_rgb(0.0, 0.0, 0.0);
        PixelContext::new(cx).rectangle(ll.0, ll.1, width, -height);
        cx.stroke().unwrap();
    }

    /// Set the cairo transformation matrix to plot in data coordinates
    fn transform_data(
        &self,
        cx: &Context,
        // pixel coordinates for the full Axes area (including margins):
        rect: gtk::cairo::Rectangle,
    ) {
        // physical size of the chart area:
        let width = rect.width() - self.margins.left - self.margins.right;
        let height = rect.height() - self.margins.bottom - self.margins.top;

        let scale_width = width / (self.primary_x.range.1 - self.primary_x.range.0);
        let scale_height = height / (self.primary_y.range.0 - self.primary_y.range.1);

        let origin = (
            rect.x() + self.margins.left - self.primary_x.range.0 * scale_width,
            rect.y() + rect.height() - self.margins.bottom - self.primary_y.range.0 * scale_height,
        );

        cx.transform(Matrix::new(
            scale_width,
            0.0,
            0.0,
            scale_height,
            origin.0,
            origin.1,
        ));
    }
}

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
        let xs: Vec<_> = (-250i32..=250).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_sin: Vec<_> = xs.iter().map(|x| x.sin()).collect();
        let signal_a: Vec<_> =
            xs.iter().map(|x| ((0.5 * x).powf(2.0)).sin() / (0.5 + x.abs())).collect();
        let signal_sinc: Vec<_> =
            xs.iter().map(|x| if *x == 0.0 { 1.0 } else { x.sin() / x }).collect();

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
            gtk::DrawingArea::builder().content_height(500).content_width(800).build(),
        ));

        struct SharedState {
            axes: Axes,
            /// current rectangle for the Axes (pixel coords), updated on draw:
            current_rect: Rectangle,
            /// cursor position from last motion event
            cursor: CursorPosition,
        }

        let state = Rc::new(RefCell::new(SharedState {
            axes: Axes::new(Extents {
                xmin: -2.0 * PI,
                xmax: 2.0 * PI,
                ymin: -1.1,
                ymax: 1.1,
            }),
            current_rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cursor: CursorPosition::None,
        }));

        // let current_rect = Rc::new(RefCell::new(Rectangle::new(0.0, 0.0, 1.0, 1.0)));

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
            let cursor = st.borrow().axes.cursor_position(st.borrow().current_rect, x, y);
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

        example_signals(&mut state.borrow_mut().axes);

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
