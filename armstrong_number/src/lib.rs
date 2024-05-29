pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let mut sum = 0;
    let num_of_digits = nb.to_string().len() as u32;

    for digit in nb.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
        sum += digit.pow(num_of_digits);
    }

    if sum == nb {
        Some(nb)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", is_armstrong_number(0));
    println!("{:?}", is_armstrong_number(1));
    println!("{:?}", is_armstrong_number(153));
    println!("{:?}", is_armstrong_number(370));
    println!("{:?}", is_armstrong_number(371));
    println!("{:?}", is_armstrong_number(407));
    println!("{:?}", is_armstrong_number(400));
    println!("{:?}", is_armstrong_number(198));
}
