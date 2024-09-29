use adw;
use adw::prelude::*;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};

use crate::plot_component::*;

struct AppModel {
    sidebar: bool,
    counter: u8,
    plot: Controller<PlotModel>,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    ShowSidebar,
    HideSidebar,
}

#[derive(Debug)]
enum AppOutput {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = AppOutput;

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

                    pack_start = &gtk::ToggleButton {
                        set_icon_name: "dock-left",
                        set_active: true,
                        connect_toggled[sender] => move |btn| {
                            let action = if btn.is_active() { AppMsg::ShowSidebar } else {AppMsg::HideSidebar};
                            sender.input(action)
                        },
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "menu",
                    },
                },
                #[wrap(Some)]
                set_content = &adw::NavigationSplitView {
                    set_show_content: true,
                    #[watch]
                    set_collapsed: !model.sidebar,
                    #[wrap(Some)]
                    set_sidebar = &adw::NavigationPage {
                        set_title: "the sidebar",
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_spacing: 12,

                            gtk::Label {
                                set_label: "asdf"
                            }
                        }
                    },
                    #[wrap(Some)]
                    set_content = &adw::NavigationPage {
                        set_title: "the content",
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_vexpand: true,
                            set_spacing: 12,
                            append = model.plot.widget(),
                        }
                    },
                }
            }
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

        let model = AppModel {
            counter,
            plot,
            sidebar: true,
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::ShowSidebar => self.sidebar = true,
            AppMsg::HideSidebar => self.sidebar = false,
        }
    }
}

pub fn main() {
    let app = RelmApp::new("com.example.bla");
    relm4_icons::initialize_icons();
    app.run::<AppModel>(123);
}
