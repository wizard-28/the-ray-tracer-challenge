use std::ops;

use crate::float::ApproxEq;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
    pub const fn red() -> Self {
        Color::new(1.0, 0.0, 0.0)
    }
    pub const fn green() -> Self {
        Color::new(0.0, 1.0, 0.0)
    }
    pub const fn blue() -> Self {
        Color::new(0.0, 0.0, 1.0)
    }
    pub const fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red.approx_eq_at_low_precision(other.red)
            && self.green.approx_eq_at_low_precision(other.green)
            && self.blue.approx_eq_at_low_precision(other.blue)
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

// Hadamard product
impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25),
            Color::new(1.6, 0.7, 1.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25),
            Color::new(0.2, 0.5, 0.5)
        )
    }

    #[test]
    fn scale() {
        assert_eq!(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
        assert_eq!(2.0 * Color::new(0.2, 0.3, 0.4), Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn mul() {
        assert_eq!(
            Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1),
            Color::new(0.9, 0.2, 0.04)
        );
    }
}
