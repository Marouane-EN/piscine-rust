use std::ops::Add;
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

impl<T: Scalar<Item = T>> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let result: Vec<T> = self.0.iter()
            .zip(rhs.0.iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar<Item = T>> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let dot_product = self.0.iter()
            .zip(rhs.0.iter())
            .map(|(&a, &b)| a * b)
            .fold(T::zero(), |acc, x| acc + x);

        Some(dot_product)
    }
}