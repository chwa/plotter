use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use gtk::cairo::Rectangle;

use crate::{
    axes::{Axes, Extents},
    axis::Axis,
};

struct Plot {
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
        println!("plot.draw()");

        let mut y = rect.y();
        for (ax, h) in self.axes.iter() {
            println!("ax {y}");
            let row_height_px = (*h / h_sum * rect.height()).round();
            ax.borrow_mut().draw(
                cx,
                gtk::cairo::Rectangle::new(rect.x(), y, rect.width(), row_height_px),
            );
            y += row_height_px;
        }
    }

    fn add_axes(&mut self) -> Rc<RefCell<Axes>> {
        let axes = Rc::new(RefCell::new(Axes::new(Extents {
            xmin: -2.0 * PI,
            xmax: 2.0 * PI,
            ymin: -1.1,
            ymax: 1.1,
        })));
        self.axes.push((axes.clone(), 1.0));
        axes
    }
}

pub mod demo {
    use crate::axes::{AxesCursorPosition, Extents, Trace};

    use super::*;
    use gtk::{cairo::Rectangle, prelude::*};
    use std::{cell::RefCell, f64::consts::PI, rc::Rc};

    pub fn main() -> gtk::glib::ExitCode {
        let app = gtk::Application::builder().build();

        app.connect_activate(build_ui);
        app.run()
    }

    struct CursorPosition {
        axes: Option<usize>,
        ax_pos: AxesCursorPosition,
    }

    // struct SharedState {
    //     axes: Axes,
    //     /// current rectangle for the Axes (pixel coords), updated on draw:
    //     current_rect: Rectangle,
    //     /// cursor position from last motion event
    //     cursor: CursorPosition,
    // }

    // struct PlotWindow {
    //     darea: gtk::DrawingArea,
    //     axes:
    //     /// current rectangle for the Axes (pixel coords), updated on draw:
    //     current_rect: Rectangle,
    //     /// cursor position from last motion event
    //     cursor: CursorPosition,
    // }

    // impl PlotWindow {
    // fn new() -> Rc<RefCell<Self>> {
    //     Rc::new(RefCell::new(Self {
    //         darea: gtk::DrawingArea::builder().content_height(500).content_width(800).build(),
    //         current_rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
    //         cursor: CursorPosition {
    //             axes: None,
    //             ax_pos: AxesCursorPosition::None,
    //         },
    //     }))
    // }

    fn example1(axes: &mut Axes) {
        let xs: Vec<_> = (-250i32..=250).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_sin: Vec<_> = xs.iter().map(|x| x.sin()).collect();

        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_sin).collect(),
            "Signal",
        ));
    }

    fn example2(axes: &mut Axes) {
        let xs: Vec<_> = (-250i32..=250).map(|x| x as f64 * 0.01 * PI).collect();
        let signal_a: Vec<_> =
            xs.iter().map(|x| ((0.5 * x).powf(2.0)).sin() / (0.5 + x.abs())).collect();
        let signal_sinc: Vec<_> =
            xs.iter().map(|x| if *x == 0.0 { 1.0 } else { x.sin() / x }).collect();

        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_a).collect(),
            "Signal",
        ));
        axes.add_trace(Trace::new(
            std::iter::zip(xs.clone(), signal_sinc).collect(),
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
            cursor: AxesCursorPosition,
        }

        let state = Rc::new(RefCell::new(SharedState {
            plot: Plot::new(),
            current_rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cursor: AxesCursorPosition::None,
        }));

        let st = state.clone();
        let position = gtk::cairo::Rectangle::new(0.0, 0.0, 1.0, 1.0);
        darea.borrow().set_draw_func(move |_da, cx, width, height| {
            cx.set_source_rgb(1.0, 0.9, 1.0);
            cx.paint().unwrap();

            // let a = st.borrow().plot.axes[0].0.borrow().;

            st.borrow_mut().current_rect = Rectangle::new(
                position.x() * width as f64,
                position.y() * height as f64,
                position.width() * width as f64,
                position.height() * height as f64,
            );

            let rect = st.borrow().current_rect;
            dbg!(rect);
            st.borrow_mut().plot.draw(cx, rect);
        });

        let ax = state.borrow_mut().plot.add_axes();
        example1(&mut ax.borrow_mut());
        let ax = state.borrow_mut().plot.add_axes();
        example2(&mut ax.borrow_mut());

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
    // }
}
