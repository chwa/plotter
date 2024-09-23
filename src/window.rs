use adw;
use adw::prelude::*;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    RelmWidgetExt, SimpleComponent,
};

use crate::plot_component::*;

struct AppModel {
    counter: u8,

    plot: Controller<PlotModel>,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        adw::Window {
            set_default_width: 800,
            set_default_height: 500,

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "My HeaderBar",
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                    },
                },
                #[wrap(Some)]
                set_content = &gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: true,
                    // set_spacing: 12,

                    #[wrap(Some)]
                    set_start_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_spacing: 12,

                        gtk::Label {
                            set_label: "asdf"
                        }

                        // #[local_ref]
                        // avatar -> adw::Avatar,

                        // model.album.widget(),
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Paned {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,

                        #[wrap(Some)]
                        set_start_child = &gtk::Frame {
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: true,
                                set_spacing: 12,

                                append = model.plot.widget(),
                            },
                        },

                        #[wrap(Some)]
                        set_end_child = &gtk::Frame {
                            gtk::Box{
                                set_orientation: gtk::Orientation::Vertical,
                                set_vexpand: false,
                                set_spacing: 12,

                                gtk::Label {
                                    set_label: "Bla"
                                },
                            }
                        },

                    }

                    // #[local_ref]
                    // avatar -> adw::Avatar,

                    // model.album.widget(),
                }
            }
            // set_title: Some("Simple app"),
            // set_default_width: 300,
            // set_default_height: 100,

            // #[name = "navigation"]
            // adw::NavigationSplitView {
            //     adw::NavigationPage {
            //         set_title: "Navi title"
            //     },
            //     adw::NavigationPage {

            //         gtk::Box {
            //             set_orientation: gtk::Orientation::Vertical,
            //             set_spacing: 5,
            //             set_margin_all: 5,

            //             gtk::Button {
            //                 set_label: "Increment",
            //                 connect_clicked => AppMsg::Increment
            //             },

            //             gtk::Button::with_label("Decrement") {
            //                 connect_clicked => AppMsg::Decrement
            //             },

            //             gtk::Label {
            //                 #[watch]
            //                 set_label: &format!("Counter: {}", model.counter),
            //                 set_margin_all: 5,
            //             }
            //         }
            //     }
            // }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let plot: Controller<PlotModel> =
            PlotModel::builder().launch(()).forward(sender.input_sender(), |msg| match msg {
                PlotOutput::Close => AppMsg::Increment,
            });

        let model = AppModel { counter, plot };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

pub fn main() {
    // let app = adw::Application::builder().application_id("com.example.app").build();
    // let app = RelmApp::from_app(app);
    // app.run::<AppModel>(0);
    let app = RelmApp::new("relm4.example.bla");
    app.run::<AppModel>(123);
}
