pub trait Locator {
    fn get_ticks(&self, range: (f64, f64), min_spacing: Option<f64>)
        -> (Vec<f64>, Vec<f64>, usize);
}

pub struct LinLocator {
    // minimum physical tick distance as a fraction of the axis length:
    min_distance: f64,
    steps: Vec<f64>,
    minor: Vec<f64>, // matching minor steps
}

impl LinLocator {
    pub fn new(min_distance: f64) -> Self {
        Self {
            min_distance,
            steps: vec![1.0, 2.0, 2.5, 5.0, 10.0],
            minor: vec![0.2, 1.0, 0.5, 1.0, 2.0],
        }
    }
}

impl Default for LinLocator {
    fn default() -> Self {
        Self::new(0.1)
    }
}

impl Locator for LinLocator {
    fn get_ticks(
        &self,
        range: (f64, f64),
        min_distance: Option<f64>,
    ) -> (Vec<f64>, Vec<f64>, usize) {
        // shortest expected distance between ticks
        let min_dv = min_distance.unwrap_or(self.min_distance) * (range.1 - range.0);

        let exponent = min_dv.log10().floor() as i32; // TODO: overflow
        let scale10 = 10.0_f64.powi(exponent);

        let min_dv = min_dv / scale10; // min_dv is now in [1.0, 10.0) and scale10 is the power of 10

        let mut decimals = (-exponent).max(0).unsigned_abs() as usize;

        let step_digit = *self.steps.iter().find(|&&x| x >= min_dv).unwrap() as f64;

        if step_digit == 10.0 {
            decimals = decimals.saturating_sub(1);
        }
        // hack for non-integer values in 'self.steps'
        else if (step_digit - step_digit.round()).abs() > 0.1 {
            decimals += 1;
        }

        let major_step = step_digit * scale10;
        let minor_step =
            self.minor[self.steps.iter().position(|x| *x == step_digit).unwrap()] * scale10;

        let major_start = (range.0 / major_step).ceil() * major_step;
        let minor_start = (range.0 / minor_step).ceil() * minor_step;
        let nsteps_major = ((range.1 - major_start) / major_step) as i32 + 1;
        let nsteps_minor = ((range.1 - minor_start) / minor_step) as i32 + 1;

        let ticks_major: Vec<_> = (0..nsteps_major)
            .map(move |x| major_start + x as f64 * major_step)
            .collect();
        let ticks_minor = (0..nsteps_minor)
            .map(move |x| minor_start + x as f64 * minor_step)
            .filter(|x| {
                !ticks_major
                    .iter()
                    .any(|m| ((m - x) / (x + 1e-33)).abs() < 1e-12)
            }) // TODO FIXME
            .collect();
        (ticks_major, ticks_minor, decimals)
    }
}

pub struct LogLocator {}

impl Default for LogLocator {
    fn default() -> Self {
        Self {}
    }
}

impl Locator for LogLocator {
    fn get_ticks(
        &self,
        range: (f64, f64),
        min_spacing: Option<f64>,
    ) -> (Vec<f64>, Vec<f64>, usize) {
        let decades = (range.1 / range.0).log10();

        let mut exp = range.0.log10().floor() as i32;
        let mut mant = (range.0 / 10.0_f64.powi(exp)).ceil() as i32;

        let mut ticks_major: Vec<f64> = Vec::new();
        let mut ticks_minor: Vec<f64> = Vec::new();

        while mant as f64 * 10.0_f64.powi(exp) <= range.1 {
            if mant == 10 {
                mant = 1;
                exp += 1;
            }

            if mant == 1 {
                ticks_major.push(10.0_f64.powi(exp));
            } else {
                ticks_minor.push(mant as f64 * 10.0_f64.powi(exp));
            }

            mant += 1;
        }

        let decimals = 1;

        (ticks_major, ticks_minor, decimals)
    }
}
