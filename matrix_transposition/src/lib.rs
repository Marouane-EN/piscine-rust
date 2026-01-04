#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));
pub fn transpose(m: Matrix) -> Matrix {
    let new_matrix = Matrix((m.0.0, m.1.0), (m.0.1, m.1.1));

    new_matrix
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = transpose(Matrix((1, 3), (4, 5)));
        assert_eq!(matrix, Matrix((1, 4), (3, 5)));
    }
}
