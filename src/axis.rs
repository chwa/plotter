use crate::cairo_utils::{text_aligned, PixelContext, TextPos};
use crate::locator::{LinLocator, Locator, LogLocator};

#[derive(Clone, Copy)]
pub enum AxisPlacement {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Clone, Copy)]
pub enum AxisType {
    Lin,
    Log,
}

pub struct Axis {
    placement: AxisPlacement,
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

    pub fn new(place: AxisPlacement, axis_type: AxisType, range: (f64, f64)) -> Self {
        Self {
            placement: place,
            axis_type,
            range,
            label: Some(String::from("<please edit>")),
            locator: match axis_type {
                AxisType::Lin => Box::new(LinLocator::default()),
                AxisType::Log => Box::new(LogLocator::default()),
            },
        }
    }

    pub fn zoom_at(&mut self, x_01: f64, scale: f64) {
        let new_width = scale * (self.range.1 - self.range.0);
        let x_data = self.axis_to_data(x_01);

        match self.axis_type {
            AxisType::Lin => {
                self.range.0 = x_data - x_01 * new_width;
                self.range.1 = x_data + (1.0 - x_01) * new_width;
            }
            AxisType::Log => {
                let left = self.axis_to_data((1.0 - scale) * x_01);
                let right = self.axis_to_data(1.0 + (scale - 1.0) * (1.0 - x_01));

                self.range.0 = left;
                self.range.1 = right;
            }
        }
    }

    pub fn draw(&self, cx: &gtk::cairo::Context, start_pos: (f64, f64), length: f64) {
        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        cx.set_line_width(1.0);

        let (ticks_major, ticks_minor, decimals);
        match self.placement {
            AxisPlacement::Left | AxisPlacement::Right => {
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
            _ => {
                (ticks_major, ticks_minor, decimals) =
                    self.locator.get_ticks(self.range, Some(50.0 / length));
            }
        }

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        let prec = match self.axis_type {
            AxisType::Lin => Some(decimals),
            AxisType::Log => None,
        };
        self.draw_ticks(cx, length, ticks_major, 8.0, true, prec);

        PixelContext::new(cx).move_to(start_pos.0, start_pos.1);
        self.draw_ticks(cx, length, ticks_minor, 3.0, false, None);

        if let Some(text) = &self.label {
            match self.placement {
                AxisPlacement::Left => {
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
                AxisPlacement::Right => {}
                AxisPlacement::Bottom => {
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
                AxisPlacement::Top => {}
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
        decimals: Option<usize>,
    ) {
        // save start position
        let start_point = cx.current_point().unwrap();

        for t in ticks {
            let t_01 = self.data_to_axis(t);

            let text = match decimals {
                Some(precision) => {
                    if precision > 4 {
                        format!("{t:.1e}")
                    } else {
                        format!("{:.prec$}", t, prec = precision)
                    }
                }
                None => {
                    if t.log10().abs() > 3.0 {
                        format!("{t:e}")
                    } else {
                        format!("{t}")
                    }
                }
            };

            cx.move_to(start_point.0, start_point.1);

            match self.placement {
                AxisPlacement::Left => {
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
                AxisPlacement::Right => {
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
                AxisPlacement::Top => {
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
                AxisPlacement::Bottom => {
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
