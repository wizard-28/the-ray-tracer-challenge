pub mod primitive {
    pub use color::Color;
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Transform;
    pub use tuple::Tuple;
    pub use vector::Vector;

    mod color;
    mod matrix;
    mod point;
    mod tuple;
    mod vector;
}

pub mod float {
    pub use approx_eq::ApproxEq;

    mod approx_eq;
}

pub mod canvas;
