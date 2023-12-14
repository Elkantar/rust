use std::ops::Mul;

pub mod matrix;
pub mod scalar;

pub use crate::matrix::Matrix;
pub use crate::scalar::Scalar;

impl<T> Matrix<T>
where
    T: Scalar<Item = T> + Clone,
{
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {
            self.0[n].clone()
        } else {
            Vec::new()
        }
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        if self.number_of_rows() == 0 || n >= self.number_of_cols() {
            return Vec::new();
        }

        let mut col_data = Vec::with_capacity(self.number_of_rows());
        for i in 0..self.number_of_rows() {
            col_data.push(self.0[i][n].clone());
        }

        col_data
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        let num_rows_self = self.number_of_rows();
        let num_cols_self = self.number_of_cols();
        let num_cols_other = other.number_of_cols();

        if num_cols_self != other.number_of_rows() {
            return None; // Matrices cannot be multiplied
        }

        let mut result = Matrix::zero(num_rows_self, num_cols_other);

        for i in 0..num_rows_self {
            for j in 0..num_cols_other {
                let mut sum = T::zero();
                for k in 0..num_cols_self {
                    sum = sum.clone() + (self.0[i][k].clone() * other.0[k][j].clone());
                }
                result.0[i][j] = sum;
            }
        }

        Some(result)
    }
}