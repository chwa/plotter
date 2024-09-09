use gtk;

mod axes;
mod axis;
mod cairo_utils;
mod grid;
mod locator;
mod plot;

fn main() -> gtk::glib::ExitCode {
    // axis::demo::main()
    axes::demo::main()
    // plot::demo::main()
}
