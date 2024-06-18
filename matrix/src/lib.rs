extern crate lalgebra_scalar;
use lalgebra_scalar::Scalar;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T: Scalar<Item= T>>(pub Vec<Vec<T>>);
impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix { 0: Vec::new() }
    }
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut res = Matrix::zero(n, n);
        for i in 0..n {
            res.0[i][i] = T::one();
        }
        res
    }
}


