use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use float_cmp::ApproxEq;
use serde::Deserialize;

use crate::color::{PolarLuv, MARGIN};

#[derive(Deserialize)]
pub(crate) struct PolarLuvPalette {
    start: PolarLuv,
    end: PolarLuv,
    powerC: f64,
    powerL: f64,
    Cmax: Option<f64>,
}

impl PolarLuvPalette {
    fn linear_trajectory(i: f64, a: f64, b: f64) -> f64 {
        b - (b - a) * i
    }

    fn triangular_trajectory(i: f64, j: f64, a: f64, b: f64, max: f64) -> f64 {
        fn lte_precise(a: f64, b: f64) -> bool {
            a < b || a.approx_eq(b, MARGIN)
        }

        if lte_precise(i, j) {
            Self::linear_trajectory(i / j, max, b)
        } else {
            Self::linear_trajectory(((i - j) / (1.0 - j)).abs(), a, max)
        }
    }

    pub(crate) fn new(palette_path: &Path) -> Result<PolarLuvPalette, io::Error> {
        let palette_file = File::open(palette_path)?;
        let palette_reader = BufReader::new(palette_file);
        let palette: PolarLuvPalette = serde_json::from_reader(palette_reader)?;
        Ok(palette)
    }

    /// Map a value in the closed interval [0.0, 1.0] to a color
    pub(crate) fn map_scalar_to_color(&self, scalar: f64, reverse: bool) -> PolarLuv {
        let i = if reverse { 1.0 - scalar } else { scalar };

        let h = Self::linear_trajectory(i, self.start.h, self.end.h);

        let mut j = self.Cmax.map(|Cmax|
            (1.0 + ((Cmax - self.start.C) / (Cmax - self.end.C)).abs()).recip()
        );

        if j.is_some() && !(j.unwrap() > 0.0 || j.unwrap() < 1.0) {
            j = None;
        }

        let C = match j {
            Some(j) => Self::triangular_trajectory(
                i.powf(self.powerC),
                j,
                self.start.C,
                self.end.C,
                self.Cmax.unwrap(),
            ),
            None => {
                Self::linear_trajectory(i.powf(self.powerC), self.start.C, self.end.C)
            }
        };

        let L = Self::linear_trajectory(i.powf(self.powerL), self.start.L, self.end.L);

        PolarLuv { h, C, L }
    }
}
