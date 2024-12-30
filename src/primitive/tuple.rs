use super::Matrix;

pub trait Tuple: PartialEq + std::fmt::Debug {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

pub trait Transform {
    fn translate(self, x: f64, y: f64, z: f64) -> Self;
    fn translate_inverse(self, x: f64, y: f64, z: f64) -> Self;
    fn scale(self, x: f64, y: f64, z: f64) -> Self;
    fn scale_inverse(self, x: f64, y: f64, z: f64) -> Self;
    fn rotate_x(self, rad: f64) -> Self;
    fn rotate_x_inverse(self, rad: f64) -> Self;
    fn rotate_y(self, rad: f64) -> Self;
    fn rotate_y_inverse(self, rad: f64) -> Self;
    fn rotate_z(self, rad: f64) -> Self;
    fn rotate_z_inverse(self, rad: f64) -> Self;
    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self;
}

impl<T: Tuple> Transform for T {
    fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z) * self
    }
    fn translate_inverse(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::translation(x, y, z).inverse().unwrap() * self
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z) * self
    }

    fn scale_inverse(self, x: f64, y: f64, z: f64) -> Self {
        Matrix::scaling(x, y, z).inverse().unwrap() * self
    }

    fn rotate_x(self, rad: f64) -> Self {
        Matrix::rotation_x(rad) * self
    }

    fn rotate_x_inverse(self, rad: f64) -> Self {
        Matrix::rotation_x(rad).inverse().unwrap() * self
    }

    fn rotate_y(self, rad: f64) -> Self {
        Matrix::rotation_y(rad) * self
    }

    fn rotate_y_inverse(self, rad: f64) -> Self {
        Matrix::rotation_y(rad).inverse().unwrap() * self
    }

    fn rotate_z(self, rad: f64) -> Self {
        Matrix::rotation_z(rad) * self
    }

    fn rotate_z_inverse(self, rad: f64) -> Self {
        Matrix::rotation_z(rad).inverse().unwrap() * self
    }

    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::shear(xy, xz, yx, yz, zx, zy) * self
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::Transform;
    use crate::primitive::*;
    use std::f64::consts::PI;

    #[rstest]
    #[case(Point::new(-3.0,4.0,5.0), Point::new(2.0,1.0,7.0), false)]
    #[case(Point::new(-3.0,4.0,5.0), Point::new(-8.0,7.0,3.0), true)]
    #[case(Vector::new(-3.0,4.0,5.0), Vector::new(-3.0,4.0,5.0), false)]
    fn translate<T: Tuple>(#[case] x: T, #[case] result: T, #[case] inverse: bool) {
        if inverse {
            assert_eq!(x.translate_inverse(5.0, -3.0, 2.0), result);
        } else {
            assert_eq!(x.translate(5.0, -3.0, 2.0), result);
        }
    }

    #[rstest]
    #[case(Point::new(-4.0,6.0,8.0), Point::new(-8.0,18.0,32.0), false)]
    #[case(Vector::new(-4.0,6.0,8.0), Vector::new(-8.0,18.0,32.0), false)]
    #[case(Vector::new(-4.0,6.0,8.0), Vector::new(-2.0,2.0,2.0), true)]
    fn scale<T: Tuple>(#[case] x: T, #[case] result: T, #[case] inverse: bool) {
        if inverse {
            assert_eq!(x.scale_inverse(2.0, 3.0, 4.0), result);
        } else {
            assert_eq!(x.scale(2.0, 3.0, 4.0), result);
        }
    }

    #[test]
    fn reflect() {
        assert_eq!(
            Point::new(2.0, 3.0, 4.0).scale(-1.0, 1.0, 1.0),
            Point::new(-2.0, 3.0, 4.0)
        );
    }

    #[rstest]
    #[case(PI / 4.0, Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0), false)]
    #[case(PI / 2.0, Point::new(0.0, 0.0, 1.0), false)]
    #[case(PI / 4.0, Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0), true)]
    fn rotate_x(#[case] rad: f64, #[case] result: Point, #[case] inverse: bool) {
        if inverse {
            assert_eq!(Point::new(0.0, 1.0, 0.0).rotate_x_inverse(rad), result);
        } else {
            assert_eq!(Point::new(0.0, 1.0, 0.0).rotate_x(rad), result);
        }
    }

    #[rstest]
    #[case(PI / 4.0, Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0))]
    #[case(PI / 2.0, Point::new(1.0, 0.0, 0.0))]
    fn rotate_y(#[case] rad: f64, #[case] result: Point) {
        assert_eq!(Point::new(0.0, 0.0, 1.0).rotate_y(rad), result);
    }

    #[rstest]
    #[case(PI / 4.0, Point::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0))]
    #[case(PI / 2.0, Point::new(-1.0, 0.0, 0.0))]
    fn rotate_z(#[case] rad: f64, #[case] result: Point) {
        assert_eq!(Point::new(0.0, 1.0, 0.0).rotate_z(rad), result);
    }

    #[rstest]
    #[case(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, Point::new(5.0, 3.0, 4.0))]
    #[case(0.0, 1.0, 0.0, 0.0, 0.0, 0.0, Point::new(6.0, 3.0, 4.0))]
    #[case(0.0, 0.0, 1.0, 0.0, 0.0, 0.0, Point::new(2.0, 5.0, 4.0))]
    #[case(0.0, 0.0, 0.0, 1.0, 0.0, 0.0, Point::new(2.0, 7.0, 4.0))]
    #[case(0.0, 0.0, 0.0, 0.0, 1.0, 0.0, Point::new(2.0, 3.0, 6.0))]
    #[case(0.0, 0.0, 0.0, 0.0, 0.0, 1.0, Point::new(2.0, 3.0, 7.0))]
    fn shear(
        #[case] xy: f64,
        #[case] xz: f64,
        #[case] yx: f64,
        #[case] yz: f64,
        #[case] zx: f64,
        #[case] zy: f64,
        #[case] result: Point,
    ) {
        assert_eq!(
            Point::new(2.0, 3.0, 4.0).shear(xy, xz, yx, yz, zx, zy),
            result
        );
    }

    #[test]
    fn chain() {
        let p = Point::new(1.0, 0.0, 1.0);
        let p2 = p.rotate_x(PI / 2.0);
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        let p3 = p2.scale(5.0, 5.0, 5.0);
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        let p4 = p3.translate(10.0, 5.0, 7.0);
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }
}
