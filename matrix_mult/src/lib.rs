use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
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
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..self.number_of_rows() {
            for j in 0..rhs.number_of_cols() {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {
                    sum = sum + (self.0[i][k] * rhs.0[k][j]);
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
