use std::ops::{Add, AddAssign, Mul, MulAssign};
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {

    pub fn number_of_cols(&self) -> usize { 
        println!("{}", self.0.len());
        self.0[0].len() 
    }

    pub fn number_of_rows(&self) -> usize {
        println!("{}", self.0[0].len());
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        // println!("{}", self.0.iter().map(|x| x[n].clone()).collect());
        self.0.iter().map(|x| x[n].clone()).collect()
    }
}
impl<T> Mul for Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Mul<Output = T> + AddAssign + MulAssign,
{
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }
        let mut result = vec![];
        for i in 0..self.number_of_rows() {
            let mut row = vec![];
            for j in 0..other.number_of_cols() {
                let sum = self
                    .row(i)
                    .iter()
                    .zip(other.col(j).iter())
                    .fold(T::default(), |acc, (a, b)| acc + a.clone() * b.clone());
                row.push(sum);
            }
            result.push(row);
        }
        Some(Matrix(result))
    }
}


fn main() {
	let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
	println!("{:?}", matrix.col(0));
	println!("{:?}", matrix.row(1));

	let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
	let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
	let mult = matrix_1.clone() * matrix_2.clone();
	println!("{:?}", mult);
	println!("{:?}", matrix_1.number_of_cols());
	println!("{:?}", matrix_2.number_of_rows());
}
