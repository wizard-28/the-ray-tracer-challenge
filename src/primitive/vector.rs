use std::ops;

use super::Tuple;
use crate::float::ApproxEq;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

// "Cross" product
impl ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// "Dot" product (or "scalar" product)
impl ops::BitXor for Vector {
    type Output = f64;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Vector::new(3.0, -2.0, 5.0) + Vector::new(-2.0, 3.0, 1.0),
            Vector::new(1.0, 1.0, 6.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector::new(3.0, 2.0, 1.0) - Vector::new(5.0, 6.0, 7.0),
            Vector::new(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn scale() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0) * 3.5,
            Vector::new(3.5, -7.0, 10.5)
        );
        assert_eq!(
            3.5 * Vector::new(1.0, -2.0, 3.0),
            Vector::new(3.5, -7.0, 10.5)
        );
    }

    #[test]
    fn div() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0) / 2.0,
            Vector::new(0.5, -1.0, 1.5)
        );
    }

    #[rstest]
    #[case(Vector::new(1.0, 0.0, 0.0), 1.0)]
    #[case(Vector::new(0.0, 1.0, 0.0), 1.0)]
    #[case(Vector::new(0.0, 0.0, 1.0), 1.0)]
    #[case(Vector::new(1.0, 2.0, 3.0), f64::sqrt(14.0))]
    #[case(Vector::new(-1.0, -2.0, -3.0), f64::sqrt(14.0))]
    fn magnitude(#[case] vector: Vector, #[case] mag: f64) {
        assert_eq!(vector.magnitude(), mag);
    }

    #[rstest]
    #[case(Vector::new(4.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0))]
    #[case(Vector::new(1.0, 2.0, 3.0), Vector::new(1.0/f64::sqrt(14.0), 2.0/f64::sqrt(14.0),  3.0/f64::sqrt(14.0)))]
    fn normalize(#[case] vector: Vector, #[case] norm: Vector) {
        assert_eq!(vector.normalize(), norm);
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0) ^ Vector::new(2.0, 3.0, 4.0),
            20.0
        );
    }

    #[rstest]
    #[case(Vector::new(1.0, 2.0, 3.0), Vector::new(2.0, 3.0, 4.0), Vector::new(-1.0, 2.0, -1.0))]
    #[case(Vector::new(2.0, 3.0, 4.0), Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, -2.0, 1.0))]
    fn cross(#[case] a: Vector, #[case] b: Vector, #[case] product: Vector) {
        assert_eq!(a * b, product);
    }
}
