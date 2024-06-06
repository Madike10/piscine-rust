//Write a function called invert_sentence that takes a string as input and returns the words in the string in reverse order.
// In other words, the function should take a sentence as input and return a new sentence with the words reversed.
pub fn invert_sentence(string: &str) -> String {
    string.split(' ').rev().collect::<Vec<&str>>().join(" ")
}