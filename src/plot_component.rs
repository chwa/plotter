use std::{cell::RefCell, rc::Rc};

use cairo::Rectangle;
use gtk::prelude::*;
use relm4::*;

use crate::{axes::Axes, plot::Plot};

pub struct PlotModel {
    hidden: bool,
    plot: Rc<RefCell<Plot>>,
}

#[derive(Debug)]
pub enum PlotInput {
    Show,
    Accept,
    Cancel,
}

#[derive(Debug)]
pub enum PlotOutput {
    Close,
}

#[relm4::component(pub)]
impl SimpleComponent for PlotModel {
    type Init = ();
    type Input = PlotInput;
    type Output = PlotOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_start: 12,
            set_margin_end: 12,
            set_margin_top: 12,
            set_margin_bottom: 12,

            #[name = "da"]
            gtk::DrawingArea {
                // set_content_width: 500,
                // set_content_height: 200,
                set_vexpand: true,
            },

            gtk::Box {
                add_css_class: "linked",
                set_orientation: gtk::Orientation::Horizontal,

                #[name(truebtn)]
                gtk::ToggleButton {
                    set_label: "True",

                },

                #[name(falsebtn)]
                gtk::ToggleButton {
                    set_label: "False",
                    set_group: Some(&truebtn),
                },
            }
        }

    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PlotModel {
            hidden: false,
            plot: Rc::new(RefCell::new(Plot::new())),
        };

        let widgets = view_output!();

        let plot = model.plot.clone();
        let axes1 = Axes::linear(None);
        plot.borrow_mut().add_axes(axes1.clone());

        widgets.da.set_draw_func(move |_da, cx, width, height| {
            cx.set_source_rgb(1.0, 0.9, 1.0);
            cx.paint().unwrap();
            let rect = Rectangle::new(0.0, 0.0, width as f64, height as f64);
            plot.borrow().draw(cx, rect);
        });

        ComponentParts { model, widgets }
    }
}
