#[derive(Debug)]
pub struct Matrix<const S: usize>([[f64; S]; S]);

impl<const S: usize> Default for Matrix<S> {
    fn default() -> Self {
        Self([[0.0; S]; S])
    }
}
