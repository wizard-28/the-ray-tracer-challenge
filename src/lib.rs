pub mod primitive {
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;

    mod point;
    mod tuple;
    mod vector;
}

pub mod float {
    pub use approx_eq::ApproxEq;

    mod approx_eq;
}
