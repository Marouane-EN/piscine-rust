use scalar::Scalar;
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T: Scalar>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Self([[T::zero(); w]; H])
    }
}

impl<const S: usize, T: Scalar> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut raw = [[T::zero(); S]; S];
        for i in 0..S {
            raw[i][i] = T::one();
        }

        Matrix(raw)
    }
}
