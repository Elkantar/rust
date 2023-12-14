mod scalar;
use scalar::Scalar;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        let data = vec![vec![T::zero(); 4]; 3];
        Matrix(data)
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let data = vec![vec![T::zero(); col]; row];
        Matrix(data)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            data[i][i] = T::one();
        }
        Matrix(data)
    }
}

// Add PartialEq implementation
impl<T> PartialEq for Matrix<T>
where
    T: PartialEq, // Ensure T can be compared for equality
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
