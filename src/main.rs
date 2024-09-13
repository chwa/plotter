use gtk;

mod axes;
mod axis;
mod cairo_utils;
mod grid;
mod locator;
mod plot;
// mod relmplot;

fn main() -> gtk::glib::ExitCode {
    // axis::demo::main()
    // axes::demo::main()
    // relmplot::main()
    plot::demo::main()
}
