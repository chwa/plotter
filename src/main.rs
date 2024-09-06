use gtk;

mod axes;
mod axis;
mod cairo_utils;
mod grid;
mod locator;
// mod plot;

fn main() -> gtk::glib::ExitCode {
    // axis::demo::main()
    axes::demo::main()
}

// let gesture = GestureDrag::new();
// gesture.set_button(BUTTON_MIDDLE);

// let ax2 = ax.clone();
// let da2 = darea.clone();
// gesture.connect_drag_update(move |_, x, y| {
//     dbg!(x, y);
//     ax2.borrow_mut().shift(x, y);
//     da2.borrow().queue_draw();
// });
// darea.borrow().add_controller(gesture);

// let zoom = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
// let ax2 = ax.clone();
// let da2 = darea.clone();
// zoom.connect_scroll(move |_, _, y| {
//     dbg!(y);
//     ax2.borrow_mut().zoom_at_cursor(1.02 + 0.04 * y);
//     da2.borrow().queue_draw();
//     glib::Propagation::Stop
// });
// darea.borrow().add_controller(zoom);

// let motion = EventControllerMotion::new();
// let ax2 = ax.clone();
// motion.connect_motion(move |_, x, y| {
//     // dbg!((x, y));
//     ax2.borrow_mut().move_cursor(x, y);
// });

// darea.borrow().add_controller(motion);
// // let controller = EventControllerScroll::builder()
// //     .flags(EventControllerScrollFlags::BOTH_AXES)
// //     .build();

// // controller.connect("scroll", true, |x| {
// //     dbg!(x);
// //     println!("scroll!");
// //     Some(true.to_value())
// // });
