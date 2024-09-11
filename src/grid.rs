use crate::{axis::Axis, cairo_utils::PixelContext};

pub struct Grid {}

impl Grid {
    pub fn draw(
        &self,
        cx: &gtk::cairo::Context,
        rect: gtk::cairo::Rectangle,
        primary_x: &Axis,
        primary_y: &Axis,
    ) {
        cx.set_line_width(1.0);

        // move to lower left corner
        PixelContext::new(cx).move_to(rect.x(), rect.y() + rect.height());

        // save start position
        let start_point = cx.current_point().unwrap();

        let (x_ticks_major, x_ticks_minor, _) = primary_x
            .locator
            .get_ticks(primary_x.range, Some(50.0 / rect.width()));

        let (y_ticks_major, y_ticks_minor, _) = primary_y
            .locator
            .get_ticks(primary_y.range, Some(50.0 / rect.height()));

        let x_range = primary_x.range;
        let y_range = primary_y.range;

        // minor
        cx.set_source_rgb(0.925, 0.925, 0.925);
        for t in x_ticks_minor {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = primary_x.data_to_axis(t);
            PixelContext::new(cx).rel_move_to(t_01 * rect.width(), 0.0);
            PixelContext::new(cx).rel_line_to(0.0, -rect.height());
        }
        for t in y_ticks_minor {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = primary_y.data_to_axis(t);
            PixelContext::new(cx).rel_move_to(0.0, -t_01 * rect.height());
            PixelContext::new(cx).rel_line_to(rect.width(), 0.0);
        }
        cx.stroke().unwrap();

        // major
        cx.set_source_rgb(0.8, 0.8, 0.8);
        for t in x_ticks_major {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = primary_x.data_to_axis(t);
            PixelContext::new(cx).rel_move_to(t_01 * rect.width(), 0.0);
            PixelContext::new(cx).rel_line_to(0.0, -rect.height());
        }
        for t in y_ticks_major {
            cx.move_to(start_point.0, start_point.1);
            let t_01 = primary_y.data_to_axis(t);
            PixelContext::new(cx).rel_move_to(0.0, -t_01 * rect.height());
            PixelContext::new(cx).rel_line_to(rect.width(), 0.0);
        }
        cx.stroke().unwrap();
    }
}
