#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

pub(crate) mod palettes;

use float_cmp::{ApproxEq, F64Margin};
use serde::Deserialize;

pub(crate) static MARGIN: F64Margin = F64Margin { epsilon: 0.0, ulps: 1 };

/// Cylindrical transformation of CIELUV (HCL or CIELCh(uv) color space)
#[derive(Deserialize)]
pub(crate) struct PolarLuv {
    h: f64,
    C: f64,
    L: f64
}

impl PolarLuv {
    pub(crate) fn as_Luv(&self) -> Luv {
        Luv {
            L: self.L,
            u: self.C * self.h.to_radians().cos(),
            v: self.C * self.h.to_radians().sin()
        }
    }

    pub(crate) fn as_image_Rgb(&self) -> image::Rgb<u8> {
        self.as_Luv()
            .as_XYZ()
            .as_RGB()
            .as_sRGB()
            .as_image_Rgb()
    }
}

/// CIE 1976 L*, u*, v* color space (CIELUV)
pub(crate) struct Luv {
   L: f64,
   u: f64,
   v: f64
}

impl Luv {
    /// L*, u*, v* coordinates of CIE standard illuminant D65 using the 
    /// standard 2° observer
    const D65: Luv = Luv { L: 100.0, u: 0.19782_9, v: 0.46833_2 };

    pub(crate) fn as_XYZ(&self) -> XYZ {
        if self.L.approx_eq(0.0, MARGIN) {
            XYZ { X: 0.0, Y: 0.0, Z: 0.0 }
        } else {
            let u_ = self.u / (13.0 * self.L) + Self::D65.u;
            let v_ = self.v / (13.0 * self.L) + Self::D65.v;

            let Y = if self.L > 8.0 {
                ((self.L + 16.0) / 116.0).powf(3.0)
            } else {
                f64::powf(3.0 / 29.0, 3.0) * self.L
            };
            let X = Y * (9.0 * u_) / (4.0 * v_);
            let Z = Y * (12.0 - 3.0 * u_ - 20.0 * v_) / (4.0 * v_);
            XYZ { X, Y, Z }
        }
    }
}

/// CIE 1931 color space
pub(crate) struct XYZ {
    X: f64,
    Y: f64,
    Z: f64
}

impl XYZ {
    pub(crate) fn as_RGB(&self) -> RGB {
        RGB {
            R: ( 3.240479 * self.X - 1.537150 * self.Y - 0.498535 * self.Z),
            G: (-0.969256 * self.X + 1.875992 * self.Y + 0.041556 * self.Z),
            B: ( 0.055648 * self.X - 0.204043 * self.Y + 1.057311 * self.Z)
        }
    }
}

/// Rec. 709 standard for RGB color model
pub(crate) struct RGB {
    R: f64,
    G: f64,
    B: f64
}

impl RGB {
    pub(crate) fn as_sRGB(&self) -> sRGB {
        sRGB {
            R: sRGB::transfer_function(self.R),
            G: sRGB::transfer_function(self.G),
            B: sRGB::transfer_function(self.B)
        }
    }
}

/// sRGB standard as defined in IEC 61966-2-1:1999
pub(crate) struct sRGB {
    R: f64,
    G: f64,
    B: f64
}

impl sRGB {
    pub(crate) fn transfer_function(component: f64) -> f64 {
        if component > 0.0031308 {
            1.055 * component.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * component
        }
    }

    pub(crate) fn as_image_Rgb(&self) -> image::Rgb<u8> {
        fn confine_component_to_gamut(component: f64) -> f64 {
            if component < 0.0 {
                0.0
            } else if component > 255.0 {
                255.0
            } else {
                component
            }
        }

        image::Rgb([
            confine_component_to_gamut(self.R * 255.0) as u8,
            confine_component_to_gamut(self.G * 255.0) as u8,
            confine_component_to_gamut(self.B * 255.0) as u8
        ])
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use crate::{
        color::{MARGIN, PolarLuv, Luv, XYZ, RGB, sRGB},
        tests::float::{dp_eq, sf_eq},
    };

    static N_SIG_FIGS: u32 = 5;

    #[test]
    fn PolarLuv_as_Luv_test() {
        impl Luv {
            fn approx_eq(&self, other: &Luv) -> bool {
                sf_eq(self.L, other.L, N_SIG_FIGS)
                    && sf_eq(self.u, other.u, N_SIG_FIGS)
                    && sf_eq(self.v, other.v, N_SIG_FIGS)
            }
        }

        let point1 = PolarLuv { h: 0.0, C: 0.0, L: 0.0 }; // black
        assert!(point1.as_Luv().approx_eq(&Luv { L: 0.0, u: 0.0, v: 0.0 }));

        let point2 = PolarLuv { h: 300.0, C: 40.0, L: 15.0 }; // initial point in Viridis
        assert!(point2.as_Luv().approx_eq(&Luv { L: 15.0, u: 20.0, v: -34.6410 }));
    }

    #[test]
    fn Luv_as_XYZ_test() {
        impl XYZ {
            fn approx_eq(&self, other: &XYZ) -> bool {
                sf_eq(self.X, other.X, N_SIG_FIGS)
                    && sf_eq(self.Y, other.Y, N_SIG_FIGS)
                    && sf_eq(self.Z, other.Z, N_SIG_FIGS)
            }
        }

        let point1 = Luv { L: 0.0, u: 0.0, v: 0.0 };
        assert!(point1.as_XYZ().approx_eq(&XYZ { X: 0.0, Y: 0.0, Z: 0.0 }));

        let point2 = Luv { L: 15.0, u: 20.0, v: -34.6410 };
        assert!(point2.as_XYZ().approx_eq(&XYZ { X: 0.044377_2, Y: 0.019085_8, Z: 0.086752_3 }));
    }

    #[test]
    fn XYZ_as_RGB_test() {
        impl RGB {
            fn approx_eq(&self, other: &RGB) -> bool {
                dp_eq(self.R, other.R, N_SIG_FIGS)
                    && dp_eq(self.G, other.G, N_SIG_FIGS)
                    && dp_eq(self.B, other.B, N_SIG_FIGS)
            }
        }

        let point1 = XYZ { X: 0.0, Y: 0.0, Z: 0.0 };
        assert!(point1.as_RGB().approx_eq(&RGB { R: 0.0, G: 0.0, B: 0.0 }));

        let point2 = XYZ { X: 0.044377_2, Y: 0.019085_8, Z: 0.086752_3 };
        assert!(point2.as_RGB().approx_eq(&RGB { R: 0.07121_7, G: -0.00360_3, B: 0.09029_9 }));
    }

    #[test]
    fn sRGB_transfer_function_test() {
        assert!(sRGB::transfer_function(0.0).approx_eq(0.0, MARGIN));
        assert!(sf_eq(sRGB::transfer_function(0.002), 0.02584_0, 4));
        assert!(dp_eq(sRGB::transfer_function(0.004), 0.050_7, 3));
    }

    #[test]
    fn PolarLuv_as_image_Rgb_test() {
        let point1 = PolarLuv { h: 0.0, C: 0.0, L: 0.0 };
        assert_eq!(point1.as_image_Rgb(), image::Rgb([0; 3]));

        let point2 = PolarLuv { h: 300.0, C: 40.0, L: 15.0 };
        assert_eq!(point2.as_image_Rgb(), image::Rgb([75, 0, 84]));
    }
}
