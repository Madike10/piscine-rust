pub fn number_logic(num: u32) -> bool {
    let mut sum = 0;
    let num_of_digits = num.to_string().len() as u32;
    for digit in num.to_string().chars().map(|c| c.to_digit(10).unwrap()){
        sum += digit.pow(num_of_digits);
    }
    if num == sum{
        return true;
    }
    false
}
fn main() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }
}
