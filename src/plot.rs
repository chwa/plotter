use gtk::cairo::Rectangle;

use crate::{axes::Axes, axis::Axis};

// struct Axes {
//     extents: (f64, f64),

//     primary_x: Axis,
//     primary_y: Axis,
//     secondary_y: Option<Axis>,
//     margins:
// }

// impl Axes {
//     pub fn draw(&self, cx: &gtk::cairo::Context, rect: gtk::cairo::Rectangle) {}
// }

struct Plot {
    axes: Vec<(Axes, f64)>,
}

impl Plot {
    pub fn draw(&self, cx: &gtk::cairo::Context, width: i32, height: i32) {
        let h_sum: f64 = self.axes.iter().map(|(_, h)| h).sum();

        let mut y = 0.0;
        for (ax, h) in &self.axes {
            let row_height_px = (h / h_sum * height as f64).round();
            ax.draw(
                cx,
                gtk::cairo::Rectangle::new(0.0, y, width as f64, row_height_px),
            );
            y += row_height_px;
        }
    }
}
