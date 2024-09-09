use crate::cairo_utils::{text_aligned, PixelContext, TextPos};
use crate::locator::{LinLocator, Locator};

#[derive(Clone, Copy)]
enum AxisDirection {
    Left,
    Right,
    Bottom,
    Top,
}

enum AxisType {
    Lin,
    Log,
}

pub struct Axis {
    direction: AxisDirection,
    axis_type: AxisType,
    pub range: (f64, f64),
    label: Option<String>,
    pub locator: Box<dyn Locator>,
}

impl Axis {
    /// map values to relative placement on axis (0 to 1 for values in the range)
    pub fn data_to_axis(&self, v: f64) -> f64 {
        match self.axis_type {
            AxisType::Lin => (v - self.range.0) / (self.range.1 - self.range.0),
            AxisType::Log => (v / self.range.0).log2() / (self.range.1 / self.range.0).log2(),
        }
    }

    /// map relative values (0 to 1) to data values
    pub fn axis_to_data(&self, v: f64) -> f64 {
        match self.axis_type {
            AxisType::Lin => self.range.0 + v * (self.range.1 - self.range.0),
            AxisType::Log => self.range.0 * (self.range.1 / self.range.0).powf(v),
        }
    }

    pub fn linear1(range: (f64, f64)) -> Self {
        Self {
            direction: AxisDirection::Bottom,
            axis_type: AxisType::Lin,
            range,
            label: Some(String::from("Horizontal axis [units]")),
            locator: Box::new(LinLocator::new(0.025)),
        }
    }

    pub fn vertical(range: (f64, f64)) -> Self {
        Self {
            direction: AxisDirection::Left,
            axis_type: AxisType::Lin,
            range,
            label: Some(String::from("Vertical axis [units]")),
            locator: Box::new(LinLocator::new(0.025)),
        }
    }

    pub fn zoom_at(&mut self, x: f64, scale: f64) {
        let width = self.range.1 - self.range.0;
        let x_01 = (x - self.range.0) / width;
        let new_width = scale * width;

        self.range.0 = x - x_01 * new_width;
        self.range.1 = x + (1.0 - x_01) * new_width;
    }

    pub fn draw(&self, cx: &gtk::cairo::Context, start_pos: (f64, f64), length: f64) {
        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        cx.set_line_width(1.0);

        let (ticks_major, ticks_minor, decimals);
        match self.direction {
            AxisDirection::Left | AxisDirection::Right => {
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
            _ => {
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
        }

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        self.draw_ticks(cx, length, ticks_major, 8.0, true, decimals);

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        self.draw_ticks(cx, length, ticks_minor, 3.0, false, 0);

        if let Some(text) = &self.label {
            match self.direction {
                AxisDirection::Left => {
                    text_aligned(
                        cx,
                        (start_pos.0, start_pos.1 - length / 2.0),
                        &text,
                        TextPos::Left,
                        15.0,
                        50.0,
                        true,
                        true,
                    );
                }
                AxisDirection::Right => {}
                AxisDirection::Bottom => {
                    text_aligned(
                        cx,
                        (start_pos.0 + length / 2.0, start_pos.1),
                        &text,
                        TextPos::Bottom,
                        15.0,
                        30.0,
                        false,
                        true,
                    );
                }
                AxisDirection::Top => {}
            }
        }
        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
    }

    pub fn draw_ticks(
        &self,
        cx: &gtk::cairo::Context,
        length: f64,
        ticks: Vec<f64>,
        tick_size: f64,
        with_labels: bool,
        decimals: usize,
    ) {
        // save start position
        let start_point = cx.current_point().unwrap();

        for t in ticks {
            let t_01 = (t - self.range.0) / (self.range.1 - self.range.0);

            let text = format!("{:.prec$}", t, prec = decimals);

            cx.move_to(start_point.0, start_point.1);

            match self.direction {
                AxisDirection::Left => {
                    PixelContext::new(cx).rel_move_to(0.0, -t_01 * length);
                    PixelContext::new(cx).rel_line_to(-tick_size, 0.0);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 - tick_size, start_point.1 - t_01 * length),
                            &text,
                            TextPos::Left,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Right => {
                    PixelContext::new(cx).rel_move_to(0.0, -t_01 * length);
                    PixelContext::new(cx).rel_line_to(tick_size, 0.0);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + tick_size, start_point.1 - t_01 * length),
                            &text,
                            TextPos::Right,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Top => {
                    PixelContext::new(cx).rel_move_to(t_01 * length, 0.0);
                    PixelContext::new(cx).rel_line_to(0.0, -tick_size);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + t_01 * length, start_point.1 - tick_size),
                            &text,
                            TextPos::Top,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
                AxisDirection::Bottom => {
                    PixelContext::new(cx).rel_move_to(t_01 * length, 0.0);
                    PixelContext::new(cx).rel_line_to(0.0, tick_size);
                    if with_labels {
                        text_aligned(
                            cx,
                            (start_point.0 + t_01 * length, start_point.1 + tick_size),
                            &text,
                            TextPos::Bottom,
                            12.0,
                            5.0,
                            false,
                            false,
                        );
                    }
                }
            }
        }
        cx.stroke().unwrap();
    }
}
