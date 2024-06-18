use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Copy
{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}
impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self::Item {
        0.0
    }

    fn one() -> Self::Item {
        1.0
    }
}

fn main() {
	println!("{:?}", f64::zero());
	println!("{:?}", i32::zero());
	println!("{:?}", f64::one());
	println!("{:?}", i32::one());
}