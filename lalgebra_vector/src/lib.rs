use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Add<Output = Self> + Clone {}

impl<T> Scalar for T where T: Add<Output = Self> + Clone {}

impl<T> Add for Vector<T>
where
    T: Scalar<Output = T>,
{
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }
        let sum: Vec<T> = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&ref a, &ref b)| a.clone() + b.clone())
            .collect();
        Some(Vector(sum))
    }
}

impl<T: Scalar + std::ops::Mul + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    pub fn new() -> Self {
        Vector { 0: Vec::new() }
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let dot: T = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(&ref a, &ref b)| a.clone() * b.clone())
            .sum();
        Some(dot)
    }
}
