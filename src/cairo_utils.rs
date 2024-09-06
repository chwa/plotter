use core::f64;
use std::f64::consts::PI;

#[derive(Debug)]
pub enum TextPos {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn text_aligned(
    cx: &gtk::cairo::Context,
    position: (f64, f64),
    text: &str,
    mut placement: TextPos,
    fontsize: f64,
    spacing: f64,
    vertical: bool,
) {
    cx.set_font_size(fontsize);
    cx.set_source_rgb(0.0, 0.0, 0.0);
    cx.select_font_face(
        // "Open Sans", //"Roboto Condensed",
        "Roboto Light",
        gtk::cairo::FontSlant::Normal,
        gtk::cairo::FontWeight::Normal,
    );
    let te = cx.text_extents(text).unwrap();
    // let fe = cr.font_extents().unwrap();

    // dbg!((&pos, cx.current_point().unwrap(), te.height() / 2.0));

    // we really need the placement "from the text's perspective" (rotated)
    if vertical {
        placement = match placement {
            TextPos::Left => TextPos::Top,
            TextPos::Right => TextPos::Bottom,
            TextPos::Top => TextPos::Right,
            TextPos::Bottom => TextPos::Left,
        }
    }

    // Find relative position of the text reference point to achieve 'placement'
    // note: text y position needs to be integer to avoid 'blurring'
    let (dx, dy) = match placement {
        TextPos::Left => (-te.x_advance() - spacing, te.height() / 2.0),
        TextPos::Right => (spacing, te.height() / 2.0),
        TextPos::Top => (-te.x_advance() / 2.0, -spacing - 0.5),
        TextPos::Bottom => (-te.x_advance() / 2.0, te.height() + spacing + 0.5),
    };

    match vertical {
        false => {
            cx.translate((position.0 + dx).round(), (position.1 + dy).round());
        }
        true => {
            cx.identity_matrix();
            // cx.translate(-position.1 + dx, position.0 + dy);
            // cx.translate(20.0, 100.0);
            cx.translate((position.0 + dy).round(), (position.1 - dx).round());
            cx.rotate(-PI / 2.0);
        }
    }
    cx.move_to(0.0, 0.0);
    cx.show_text(text).unwrap();
    cx.stroke().unwrap();
    cx.identity_matrix();
}

pub struct PixelContext<'a> {
    cx: &'a gtk::cairo::Context,
}
impl<'a> PixelContext<'a> {
    pub fn new(cx: &'a gtk::cairo::Context) -> Self {
        Self { cx }
    }

    pub fn move_to(&self, x: f64, y: f64) {
        self.cx
            .move_to((x - 0.5).round() + 0.5, (y - 0.5).round() + 0.5)
    }
    pub fn rel_move_to(&self, x: f64, y: f64) {
        self.cx.rel_move_to(x.round(), y.round())
    }
    pub fn line_to(&self, x: f64, y: f64) {
        self.cx
            .line_to((x - 0.5).round() + 0.5, (y - 0.5).round() + 0.5)
    }
    pub fn rel_line_to(&self, x: f64, y: f64) {
        self.cx.rel_line_to(x.round(), y.round())
    }
    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        self.cx.rectangle(
            (x - 0.5).round() + 0.5,
            (y - 0.5).round() + 0.5,
            width.round(),
            height.round(),
        );
    }
}
