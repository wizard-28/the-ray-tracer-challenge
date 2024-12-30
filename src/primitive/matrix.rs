use std::ops::{Deref, DerefMut, Mul};

use crate::float::ApproxEq;

use super::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize>([[f64; C]; R]);

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(data: [[f64; C]; R]) -> Self {
        Self(data)
    }

    pub fn transpose(&self) -> Matrix<C, R> {
        let mut transpose: Matrix<C, R> = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                transpose[j][i] = self[i][j];
            }
        }

        transpose
    }
}

impl Matrix<3, 3> {
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2, 2> {
        let mut res = [[0.; 2]; 2];
        for i in 0..3 {
            if i == row {
                continue;
            };
            let ii = if i > row { i - 1 } else { i };
            for j in 0..3 {
                if j == col {
                    continue;
                }
                let jj = if j > col { j - 1 } else { j };
                res[ii][jj] = self[i][j]
            }
        }
        Matrix(res)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.;
        for j in 0..3 {
            det += self[0][j] * self.cofactor(0, j);
        }
        det
    }
}

impl Matrix<4, 4> {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        Matrix::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(rad: f64) -> Matrix<4, 4> {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, rad.cos(), -rad.sin(), 0.0],
            [0.0, rad.sin(), rad.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(rad: f64) -> Matrix<4, 4> {
        Matrix::new([
            [rad.cos(), 0.0, rad.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-rad.sin(), 0.0, rad.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(rad: f64) -> Matrix<4, 4> {
        Matrix::new([
            [rad.cos(), -rad.sin(), 0.0, 0.0],
            [rad.sin(), rad.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4, 4> {
        Matrix::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3, 3> {
        let mut res = [[0.; 3]; 3];
        for i in 0..4 {
            if i == row {
                continue;
            };
            let ii = if i > row { i - 1 } else { i };
            for j in 0..4 {
                if j == col {
                    continue;
                }
                let jj = if j > col { j - 1 } else { j };
                res[ii][jj] = self[i][j]
            }
        }
        Matrix(res)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.;
        for j in 0..4 {
            det += self[0][j] * self.cofactor(0, j);
        }
        det
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        let mut res = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                res[j][i] = self.cofactor(i, j) / det;
            }
        }
        Some(Self(res))
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut matrix: Matrix<N, N> = Matrix::default();

        for i in 0..N {
            for j in 0..N {
                if i == j {
                    matrix[i][j] = 1.0;
                }
            }
        }

        matrix
    }
}

impl Matrix<2, 2> {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .flatten()
            .zip(other.iter().flatten())
            .all(|(x, y)| x.approx_eq_at_low_precision(*y))
    }
}

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self([[0.0; C]; R])
    }
}

impl<const R: usize, const C: usize> Deref for Matrix<R, C> {
    type Target = [[f64; C]; R];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const R: usize, const C: usize> DerefMut for Matrix<R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const R: usize, const C: usize> From<[[f64; C]; R]> for Matrix<R, C> {
    fn from(source: [[f64; C]; R]) -> Self {
        Self(source)
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for f64 {
    type Output = Matrix<R, C>;

    fn mul(self, mut rhs: Matrix<R, C>) -> Self::Output {
        for j in 0..C {
            for i in 0..R {
                rhs[i][j] *= self;
            }
        }
        rhs
    }
}

impl<T: Tuple> From<T> for Matrix<4, 1> {
    fn from(value: T) -> Self {
        Self([[value.x()], [value.y()], [value.z()], [value.w()]])
    }
}

impl<T: Tuple> Mul<T> for Matrix<4, 4> {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let res = self * Matrix::<4, 1>::from(rhs);
        Self::Output::new(res[0][0], res[1][0], res[2][0])
    }
}

impl<const C1: usize, const R: usize, const C2: usize> Mul<Matrix<R, C2>> for Matrix<C1, R> {
    type Output = Matrix<C1, C2>;

    fn mul(self, rhs: Matrix<R, C2>) -> Self::Output {
        let mut res = [[0.0; C2]; C1];
        for row in 0..C1 {
            for col in 0..C2 {
                res[row][col] = (0..R).map(|m| self[row][m] * rhs[m][col]).sum();
            }
        }
        Matrix(res)
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 0, 1.0)]
    #[case(0, 3, 4.0)]
    #[case(1, 0, 5.5)]
    #[case(1, 2, 7.5)]
    #[case(2, 2, 11.0)]
    #[case(3, 0, 13.5)]
    #[case(3, 2, 15.5)]
    fn construction_and_inspection(#[case] n: usize, #[case] m: usize, #[case] val: f64) {
        let matrix: Matrix<4, 4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix[n][m], val);
    }

    #[test]
    fn eq() {
        assert_eq!(
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]),
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ])
        );
    }

    #[test]
    fn neq() {
        assert_ne!(
            Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]),
            Matrix::new([
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [8.0, 7.0, 6.0, 5.0],
                [4.0, 3.0, 2.0, 1.0],
            ])
        );
    }

    #[test]
    fn mul() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let product = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, product);
    }

    #[rstest]
    #[case(Point::new(1.0, 2.0, 3.0), Point::new(18.0, 24.0, 33.0))]
    #[case(Vector::new(1.0, 2.0, 3.0), Vector::new(14.0, 22.0, 32.0))]
    fn mul_tuple<T: Tuple>(#[case] t: T, #[case] result: T) {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(matrix * t, result);
    }

    #[test]
    fn mul_identity() {
        let identity = Matrix::identity();
        let matrix = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn transpose() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ]);
        let b = Matrix::new([
            [1.0, 5.0, 9.0],
            [2.0, 6.0, 10.0],
            [3.0, 7.0, 11.0],
            [4.0, 8.0, 12.0],
        ]);

        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn submatrix_3_x_3() {
        let mat = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let sub = Matrix::new([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(mat.submatrix(0, 2), sub);
    }

    #[test]
    fn submatrix_4_x_4() {
        let mat = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let sub = Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert_eq!(mat.submatrix(2, 1), sub);
    }

    #[test]
    fn minor_3_x_3() {
        let mat = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(mat.minor(1, 0), 25.0);
    }

    #[test]
    fn minor_4_x_4() {
        let mat = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert_eq!(mat.minor(0, 0), 0.0);
    }

    #[test]
    fn cofactor_3_x_3() {
        let mat = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(mat.cofactor(0, 0), -12.0);
        assert_eq!(mat.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3_x_3() {
        let mat = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_eq!(mat.cofactor(0, 0), 56.0);
        assert_eq!(mat.cofactor(0, 1), 12.0);
        assert_eq!(mat.cofactor(0, 2), -46.0);
        assert_eq!(mat.determinant(), -196.0);
    }

    #[test]
    fn determinant_4_x_4() {
        let mat = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(mat.cofactor(0, 0), 690.0);
        assert_eq!(mat.cofactor(0, 1), 447.0);
        assert_eq!(mat.cofactor(0, 2), 210.0);
        assert_eq!(mat.cofactor(0, 3), 51.0);
        assert_eq!(mat.determinant(), -4071.0);
    }

    #[rstest]
    #[case(Matrix::new([
    [6.0, 4.0, 4.0, 4.0],
    [5.0, 5.0, 7.0, 6.0],
    [4.0, -9.0, 3.0, -7.0],
    [9.0, 1.0, 7.0, -6.0],
    ]), true)]
    #[case(Matrix::new( [
    [-4.0, 2.0, -2.0, -3.0],
    [9.0, 6.0, 2.0, 6.0],
    [0.0, -5.0, 1.0, -5.0],
    [0.0, 0.0, 0.0, 0.0],
    ]), false)]
    fn invertible(#[case] mat: Matrix<4, 4>, #[case] res: bool) {
        assert_eq!(mat.inverse().is_some(), res);
    }

    #[rstest]
    #[case(Matrix::new([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]), Matrix::new([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]))]
    #[case(Matrix::new([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]), Matrix::new([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]))]
    #[case(Matrix::new([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]), Matrix::new([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]))]
    fn inverse(#[case] a: Matrix<4, 4>, #[case] b: Matrix<4, 4>) {
        assert_eq!(a.inverse().unwrap(), b);
    }

    #[test]
    fn multiplying_product_by_inverse() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let c = a * b;

        assert_eq!(c * b.inverse().unwrap(), a);
    }
}
