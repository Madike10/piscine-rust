use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);
//use crate::{matrix, lalgebra_scalar};
impl Add for Matrix
{
    type Output = Option<Matrix>;

    fn add(self, other: Self) -> Self::Output {
        // Check if dimensions match
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            println!("{:?} :: {:?}", self.0, other.0);
            return None;
        }
        println!("{:?} :: {:?}", self.0, other.0);

        // Perform element-wise addition
        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(row_self, row_other)| {
                row_self
                    .iter()
                    .zip(row_other.iter())
                    .map(|(&val_self, &val_other)| val_self + val_other)
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }
}

impl Sub for Matrix{
    type Output = Option<Matrix>;

    fn sub(self, other: Self) -> Self::Output {
        // Check if dimensions match
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        // Perform element-wise subtraction
        let result = self.0.iter().zip(other.0.iter()).map(|(row_self, row_other)| {
            row_self.iter().zip(row_other.iter()).map(|(&val_self, &val_other)| {
                val_self - val_other
            }).collect()
        }).collect();

        Some(Matrix(result))
    }
}



fn main() {
	let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);
}
