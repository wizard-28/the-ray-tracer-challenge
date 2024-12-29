use float_cmp::approx_eq;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
    fn approx_eq_at_low_precision(self, other: Rhs) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        approx_eq!(f64, self, other)
    }

    fn approx_eq_at_low_precision(self, other: Self) -> bool {
        approx_eq!(f64, self, other, epsilon = 1.0e-3)
    }
}

impl ApproxEq for f32 {
    fn approx_eq(self, other: Self) -> bool {
        approx_eq!(f32, self, other)
    }

    fn approx_eq_at_low_precision(self, other: Self) -> bool {
        approx_eq!(f32, self, other, epsilon = 1.0e-3)
    }
}
