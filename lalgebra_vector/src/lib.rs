pub mod scalar;

use crate::scalar::Scalar;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T> Vector<T>
where
    T: Scalar<Item = T> + Clone,
    <T as Scalar>::Item: Add<Output = <T as Scalar>::Item> + Mul<Output = <T as Scalar>::Item>,
{
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<<T as Scalar>::Item> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let mut result = <T as Scalar>::Item::zero();
            for (a, b) in self.0.iter().zip(&other.0) {
                result = result + a.clone() * b.clone();
            }
            Some(result)
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Scalar<Item = T> + Clone,
    <T as Scalar>::Item: Add<Output = <T as Scalar>::Item>,
{
    type Output = Option<Vector<T>>;

    fn add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let result = self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a.clone() + b.clone())
                .collect();

            Some(Vector(result))
        }
    }
}