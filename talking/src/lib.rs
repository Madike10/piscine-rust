pub fn talking(input: &str) -> &str {
    if input.trim().is_empty() {
        "Just say something!"
    } else if input == input.to_uppercase() && input.chars().any(|c| c.is_alphabetic()) {
        if input.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if input.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
