extern crate roman_numbers;
pub use roman_numbers::{RomanDigit, RomanNumber};
pub struct MyRomanNumber(RomanNumber);
impl Iterator for MyRomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
        // Remplacer reduce par fold pour éviter les problèmes avec les types d'agrégation
        let next_value = self
            .0
             .0
            .iter()
            .fold(0, |acc, x| acc + get_decimal_equivalent(x));
        self.0 = RomanNumber::from(next_value);
        Some(self.0.clone())
    }
}
pub fn get_decimal_equivalent(roman_digit: &RomanDigit) -> u32 {
    match roman_digit {
        RomanDigit::I => 1,
        RomanDigit::IV => 4,
        RomanDigit::V => 5,
        RomanDigit::IX => 9,
        RomanDigit::X => 10,
        RomanDigit::XL => 40,
        RomanDigit::L => 50,
        RomanDigit::XC => 90,
        RomanDigit::C => 100,
        RomanDigit::CD => 400,
        RomanDigit::D => 500,
        RomanDigit::CM => 900,
        RomanDigit::M => 1000,
        RomanDigit::Nulla => 0,
    }
}

// fn main() {
// 	let mut number = RomanNumber::from(15);

// 	println!("{:?}", number);
// 	println!("{:?}", number.next());
// }
