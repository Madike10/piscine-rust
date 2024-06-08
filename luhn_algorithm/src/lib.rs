//Create a function which checks if a number is valid per the Luhn formula.
//
// The function will receive a string and return a boolean.
//
// Invalid inputs: An empty string or a number with only one digit are considered invalid.
// Handling spaces: Spaces are accepted but have to be ignored during the calculation (in other words they won't affect the result).
//
// The Luhn formula is used to check if a number is a valid credit card number and in some other scenarios where you need to check a number fast and without accessing a database.
//
// We can summarize the formula as follow:
//
// We want to check the number 4539 3195 0343 6467
// We take every second digit starting by the right
// We multiply those digits by 2
// If the result is more than 9 we subtract 9 from it
// We sum all the digits
// If sum is evenly divisible by 10 then this number is valid
// So we will get:
//
// 4539 3195 0343 6467
// 4_3_ 3_9_ 0_4_ 6_6_: numbers to modify
// 8_6_ 6_9_ 0_8_ 3_3_: modified numbers (for numbers over 9 we subtracted 9 already)
// 8569 6195 0383 3437: the new sequence of digits
// 80: the sum of all digits
// 80 is evenly divisible by 10 so the result is true
pub fn is_luhn_formula(code: &str) -> bool {
    // Remove spaces from the code
    let code = code.replace(" ", "");

    // If the code is empty or has only one digit, it's invalid
    if code.len() <= 1 {
        return false;
    }

    // If the code contains any non-digit characters, it's invalid
    if !code.chars().all(|x| x.is_digit(10)) {
        return false;
    }

    // Collect digits in reverse order for easier processing
    let digits: Vec<u32> = code.chars().rev()
        .filter_map(|c| c.to_digit(10))
        .collect();

    // Sum of all digits after applying the Luhn formula
    let mut sum = 0;

    // Iterate over the digits and apply the Luhn transformation
    for (i, &digit) in digits.iter().enumerate() {
        let mut value = digit;
        // Double every second digit (1-based index from the right)
        if i % 2 == 1 {
            value *= 2;
            // If the result is greater than 9, subtract 9
            if value > 9 {
                value -= 9;
            }
        }
        sum += value;
    }

    // If the sum is divisible by 10, the number is valid
    sum % 10 == 0
}
