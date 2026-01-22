use std::ops::{ Add, Sub };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { i: self.i + rhs.i, j: self.j + rhs.j, k: self.k + rhs.k }
    }
}

impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { i: self.i - rhs.i, j: self.j - rhs.j, k: self.k - rhs.k }
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
