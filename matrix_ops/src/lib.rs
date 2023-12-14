pub mod matrix;
pub mod scalar;
pub use crate::matrix::Matrix;
pub use crate::scalar::Scalar;
use std::ops::{Add, Sub};

impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; // Matrices have different dimensions, cannot add
        }

        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j].clone() + other.0[i][j].clone();
            }
        }

        Some(result)
    }
}

impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None; // Matrices have different dimensions, cannot subtract
        }

        let mut result = Matrix::zero(self.0.len(), self.0[0].len());
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result.0[i][j] = self.0[i][j].clone() - other.0[i][j].clone();
            }
        }

        Some(result)
    }
}
