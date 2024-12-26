use float_cmp::approx_eq;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        approx_eq!(f64, self, other)
    }
}
