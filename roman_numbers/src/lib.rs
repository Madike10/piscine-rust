use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            4 => IV,
            5 => V,
            9 => IX,
            10 => X,
            50 => L,
            90 => XC,
            100 => C,
            400 => CD,
            500 => D,
            900 => CM,
            1000 => M,
            _ => panic!("Invalid number"),
        }
    }
}

// Extract a new function append_digits to improve the organized look of the code.
fn append_digits(digit: RomanDigit, roman_number: &mut Vec<RomanDigit>) {
    match digit {
        IV => {
            roman_number.push(I);
            roman_number.push(V);
        }
        IX => {
            roman_number.push(I);
            roman_number.push(X);
        }
        XL => {
            roman_number.push(X);
            roman_number.push(L);
        }
        XC => {
            roman_number.push(X);
            roman_number.push(C);
        }
        CD => {
            roman_number.push(C);
            roman_number.push(D);
        }
        CM => {
            roman_number.push(C);
            roman_number.push(M);
        }
        _ => roman_number.push(digit),
    }
}

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let mut num = num;
        let mut roman_number = vec![];
        let digits = [M, CM, D, CD, C, XC, L, XL, X, IX, V, IV, I];
        let roman_values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        for (i, &value) in roman_values.iter().enumerate() {
            while num >= value {
                num -= value;
                // use the new function here
                append_digits(digits[i], &mut roman_number);
            }
        }
        RomanNumber(roman_number)
    }
}
