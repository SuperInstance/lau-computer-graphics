//! Color models: RGB, HSV, and conversions.

use serde::{Deserialize, Serialize};

/// RGB color with f64 channels in [0, 1].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// From 8-bit [0, 255] channels.
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }

    /// Clamp all channels to [0, 1].
    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    /// Linear interpolation between two colors.
    pub fn lerp(a: &Rgb, b: &Rgb, t: f64) -> Self {
        Self {
            r: a.r + (b.r - a.r) * t,
            g: a.g + (b.g - a.g) * t,
            b: a.b + (b.b - a.b) * t,
        }
    }

    /// Convert to HSV.
    pub fn to_hsv(&self) -> Hsv {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == self.r {
            60.0 * (((self.g - self.b) / delta) % 6.0)
        } else if max == self.g {
            60.0 * (((self.b - self.r) / delta) + 2.0)
        } else {
            60.0 * (((self.r - self.g) / delta) + 4.0)
        };
        let h = if h < 0.0 { h + 360.0 } else { h };

        let s = if max == 0.0 { 0.0 } else { delta / max };

        Hsv { h, s, v: max }
    }
}

impl std::ops::Add for Rgb {
    type Output = Rgb;
    fn add(self, rhs: Rgb) -> Rgb {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Mul<f64> for Rgb {
    type Output = Rgb;
    fn mul(self, s: f64) -> Rgb {
        Rgb::new(self.r * s, self.g * s, self.b * s)
    }
}

/// HSV color: h in [0, 360), s and v in [0, 1].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

impl Hsv {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self { h, s, v }
    }

    /// Convert to RGB.
    pub fn to_rgb(&self) -> Rgb {
        let c = self.v * self.s;
        let x = c * (1.0 - ((self.h / 60.0) % 2.0 - 1.0).abs());
        let m = self.v - c;

        let (r1, g1, b1) = match self.h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Rgb::new(r1 + m, g1 + m, b1 + m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rgb_from_u8() {
        let c = Rgb::from_u8(255, 128, 0);
        assert_relative_eq!(c.r, 1.0);
        assert_relative_eq!(c.g, 128.0 / 255.0, epsilon = 1e-6);
        assert_relative_eq!(c.b, 0.0);
    }

    #[test]
    fn test_rgb_clamp() {
        let c = Rgb::new(1.5, -0.2, 0.5).clamp();
        assert_eq!(c, Rgb::new(1.0, 0.0, 0.5));
    }

    #[test]
    fn test_rgb_lerp() {
        let a = Rgb::new(0.0, 0.0, 0.0);
        let b = Rgb::new(1.0, 1.0, 1.0);
        let mid = Rgb::lerp(&a, &b, 0.5);
        assert_relative_eq!(mid.r, 0.5);
        assert_relative_eq!(mid.g, 0.5);
        assert_relative_eq!(mid.b, 0.5);
    }

    #[test]
    fn test_rgb_add() {
        let a = Rgb::new(0.2, 0.3, 0.4);
        let b = Rgb::new(0.1, 0.2, 0.3);
        let c = a + b;
        assert_relative_eq!(c.r, 0.3);
        assert_relative_eq!(c.g, 0.5);
        assert_relative_eq!(c.b, 0.7);
    }

    #[test]
    fn test_rgb_scale() {
        let a = Rgb::new(0.2, 0.4, 0.6);
        let b = a * 2.0;
        assert_relative_eq!(b.r, 0.4);
        assert_relative_eq!(b.g, 0.8);
        assert_relative_eq!(b.b, 1.2);
    }

    #[test]
    fn test_rgb_to_hsv_red() {
        let c = Rgb::new(1.0, 0.0, 0.0);
        let hsv = c.to_hsv();
        assert_relative_eq!(hsv.h, 0.0, epsilon = 1e-6);
        assert_relative_eq!(hsv.s, 1.0, epsilon = 1e-6);
        assert_relative_eq!(hsv.v, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_rgb_to_hsv_green() {
        let c = Rgb::new(0.0, 1.0, 0.0);
        let hsv = c.to_hsv();
        assert_relative_eq!(hsv.h, 120.0, epsilon = 1e-6);
        assert_relative_eq!(hsv.s, 1.0, epsilon = 1e-6);
        assert_relative_eq!(hsv.v, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_hsv_to_rgb_red() {
        let hsv = Hsv::new(0.0, 1.0, 1.0);
        let rgb = hsv.to_rgb();
        assert_relative_eq!(rgb.r, 1.0, epsilon = 1e-6);
        assert_relative_eq!(rgb.g, 0.0, epsilon = 1e-6);
        assert_relative_eq!(rgb.b, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_hsv_to_rgb_blue() {
        let hsv = Hsv::new(240.0, 1.0, 1.0);
        let rgb = hsv.to_rgb();
        assert_relative_eq!(rgb.r, 0.0, epsilon = 1e-6);
        assert_relative_eq!(rgb.g, 0.0, epsilon = 1e-6);
        assert_relative_eq!(rgb.b, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_roundtrip_rgb_hsv() {
        let orig = Rgb::new(0.8, 0.3, 0.5);
        let hsv = orig.to_hsv();
        let back = hsv.to_rgb();
        assert_relative_eq!(orig.r, back.r, epsilon = 1e-10);
        assert_relative_eq!(orig.g, back.g, epsilon = 1e-10);
        assert_relative_eq!(orig.b, back.b, epsilon = 1e-10);
    }

    #[test]
    fn test_hsv_black() {
        let c = Rgb::new(0.0, 0.0, 0.0);
        let hsv = c.to_hsv();
        assert_relative_eq!(hsv.s, 0.0);
        assert_relative_eq!(hsv.v, 0.0);
    }
}
