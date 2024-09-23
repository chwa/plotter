use std::{cell::RefCell, rc::Rc};

use crate::axes::{Axes, AxesCursorPosition};

#[derive(Clone, Copy, Debug)]
pub enum PlotCursorPosition {
    Axes(usize, AxesCursorPosition),
    None,
}

pub struct Plot {
    axes: Vec<(Rc<RefCell<Axes>>, f64)>,
}

impl Plot {
    pub fn new() -> Self {
        Self { axes: vec![] }
    }

    pub fn draw(
        &self,
        cx: &gtk::cairo::Context,
        // pixel coordinates for the full Plot area:
        rect: gtk::cairo::Rectangle,
    ) {
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.paint().unwrap();

        let h_sum: f64 = self.axes.iter().map(|(_, h)| h).sum();

        let mut y = rect.y();
        for (ax, h) in self.axes.iter() {
            let row_height_px = (*h / h_sum * rect.height()).round();
            ax.borrow().draw(
                cx,
                gtk::cairo::Rectangle::new(rect.x(), y, rect.width(), row_height_px),
            );
            y += row_height_px;
        }
    }

    fn export_svg(&self) {
        let svg = gtk::cairo::SvgSurface::new(800.0, 500.0, Some("abc.svg")).unwrap();
        let mut cx = gtk::cairo::Context::new(svg).unwrap();
        cx.set_source_rgb(1.0, 1.0, 1.0);
        cx.paint().unwrap();
        self.draw(&mut cx, gtk::cairo::Rectangle::new(0.0, 0.0, 800.0, 500.0));
    }

    pub fn add_axes(&mut self, ax: Rc<RefCell<Axes>>) {
        self.axes.push((ax, 1.0));
    }

    pub fn cursor_position(
        &self,
        rect: gtk::cairo::Rectangle,
        x: f64,
        y: f64,
    ) -> PlotCursorPosition {
        if x >= rect.x() && x <= rect.x() + rect.width() {
            let h_sum: f64 = self.axes.iter().map(|(_, h)| h).sum();

            let mut current_y = rect.y();
            for (i, (ax, h)) in self.axes.iter().enumerate() {
                let row_height_px = (*h / h_sum * rect.height()).round();

                if y >= current_y && y < current_y + row_height_px {
                    return PlotCursorPosition::Axes(
                        i,
                        ax.borrow().cursor_position(
                            gtk::cairo::Rectangle::new(
                                rect.x(),
                                current_y,
                                rect.width(),
                                row_height_px,
                            ),
                            x,
                            y,
                        ),
                    );
                }

                current_y += row_height_px;
            }
            PlotCursorPosition::None
        } else {
            PlotCursorPosition::None
        }
    }
}

pub mod demo {
    use crate::axes::Trace;

    use super::*;
    use gtk::{cairo::Rectangle, prelude::*};
    use std::{cell::RefCell, f64::consts::PI, rc::Rc};

    pub fn main() -> gtk::glib::ExitCode {
        let app = gtk::Application::builder().build();

        app.connect_activate(build_ui);
        app.run()
    }

    fn example1(axes: &mut Axes) {
        let xs: Vec<_> = (1_i32..=500).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_sin: Vec<_> = xs.iter().map(|x| x.sin()).collect();

        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_sin).collect(),
            "Signal",
        ));
    }

    fn example2(axes: &mut Axes) {
        let xs: Vec<_> = (1_i32..=500).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_a: Vec<_> = xs.iter().map(|x| 1.0 + (2.0 * x).sin()).collect();
        let signal_b: Vec<_> = xs.iter().map(|x| 1.0 + (3.0 * x).sin()).collect();

        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_a).collect(),
            "Signal",
        ));
        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_b).collect(),
            "Signal",
        ));
    }

    pub fn build_ui(app: &gtk::Application) {
        let darea = Rc::new(RefCell::new(
            gtk::DrawingArea::builder().content_height(500).content_width(800).build(),
        ));

        struct SharedState {
            plot: Plot,
            /// current rectangle for the Axes (pixel coords), updated on draw:
            current_rect: Rectangle,
            /// cursor position from last motion event
            cursor: PlotCursorPosition,
            marker: PlotCursorPosition,
        }

        let state = Rc::new(RefCell::new(SharedState {
            plot: Plot::new(),
            current_rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cursor: PlotCursorPosition::None,
            marker: PlotCursorPosition::None,
        }));

        let st = state.clone();
        let position = gtk::cairo::Rectangle::new(0.0, 0.0, 1.0, 1.0);
        darea.borrow().set_draw_func(move |_da, cx, width, height| {
            cx.set_source_rgb(1.0, 0.9, 1.0);
            cx.paint().unwrap();

            st.borrow_mut().current_rect = Rectangle::new(
                position.x() * width as f64,
                position.y() * height as f64,
                position.width() * width as f64,
                position.height() * height as f64,
            );

            let rect = st.borrow().current_rect;
            st.borrow_mut().plot.draw(cx, rect);
        });

        let axes1 = Axes::linear(None);
        state.borrow_mut().plot.add_axes(axes1.clone());
        example1(&mut axes1.borrow_mut());

        let axes2 = Axes::linear(Some(axes1.borrow().primary_x.clone()));
        state.borrow_mut().plot.add_axes(axes2.clone());
        example2(&mut axes2.borrow_mut());

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .child(&*darea.borrow())
            .build();

        // Motion event controller
        let motion = gtk::EventControllerMotion::new();
        let da = darea.clone();
        let st = state.clone();
        motion.connect_motion(move |_, x, y| {
            let cursor = st.borrow().plot.cursor_position(st.borrow().current_rect, x, y);

            if let PlotCursorPosition::Axes(i, axpos) = &cursor {
                let xy = st.borrow().plot.axes[*i].0.borrow().snap_cursor(*axpos);
                st.borrow().plot.axes[*i].0.borrow_mut().cursor = xy;
            }
            st.borrow_mut().cursor = cursor;
            da.borrow().queue_draw();
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

                let mut st_borrow = st.borrow_mut();

                if let PlotCursorPosition::Axes(i, axes_pos) = st_borrow.cursor {
                    if let Some((axes, _)) = st_borrow.plot.axes.get_mut(i) {
                        axes.borrow_mut().zoom_at(axes_pos, scale);
                    }
                }

                da.borrow().queue_draw();
            }
            gtk::glib::Propagation::Stop
        });
        darea.borrow().add_controller(zoom);

        // Key event controller
        let key = gtk::EventControllerKey::new();
        let da = darea.clone();
        let st = state.clone();
        key.connect_key_pressed(move |_, k, _, _| {
            if k == gtk::gdk::Key::from_name("f").unwrap() {
                for (ax, _) in &st.borrow().plot.axes {
                    ax.borrow_mut().zoom_fit();
                }
                da.borrow().queue_draw();
            } else if k == gtk::gdk::Key::from_name("s").unwrap() {
                st.borrow().plot.export_svg();
            }
            gtk::glib::Propagation::Stop
        });
        window.add_controller(key);

        window.present();
    }
}
