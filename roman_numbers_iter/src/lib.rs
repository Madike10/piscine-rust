use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
        let mut number: i32 = 0;
        if self.0.is_empty() {
            None
        } else {
            for i in 0..self.0.len() {
                match self.0[i] {
                    Nulla => {
                        number += 0;
                    }
                    I => {
                        if i < self.0.len() - 1 {
                            match self.0[i + 1] {
                                V => number -= 1,
                                X => number -= 1,
                                _ => number += 1,
                            }
                        } else {
                            number += 1;
                        }
                    }
                    X => {
                        if i < self.0.len() - 1 {
                            match self.0[i + 1] {
                                L => number -= 10,
                                C => number -= 10,
                                _ => number += 10,
                            }
                        } else {
                            number += 10;
                        }
                    }
                    C => {
                        if i < self.0.len() - 1 {
                            match self.0[i + 1] {
                                D => number -= 100,
                                M => number -= 100,
                                _ => number += 100,
                            }
                        } else {
                            number += 100;
                        }
                    }
                    M => number += 1000,
                    D => number += 500,
                    L => number += 50,
                    V => number += 5,
                }
            }
            Some(RomanNumber::from((number + 1) as u32))
        }
    }
}

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman digit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }
        let mut result = Vec::new();
        let divisors = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        for (i, n) in divisors.iter().enumerate() {
            while *n <= num {
                if i % 2 == 0 {
                    result.push(RomanDigit::from(*n));
                } else {
                    let rem = divisors[i - 1] - divisors[i];
                    result.push(RomanDigit::from(rem));
                    result.push(RomanDigit::from(divisors[i - 1]));
                }
                num -= *n;
            }
        }
        RomanNumber(result)
    }
}