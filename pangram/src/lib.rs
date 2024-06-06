use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {
    let mut s = s.to_lowercase();
    let mut set = HashSet::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            set.insert(c);
        }
    }
    set.len() == 26
}

fn main() {
    println!(
        "{}",
        is_pangram("the quick brown fox jumps over the lazy dog!")
    );
    println!("{}", is_pangram("this is not a pangram!"));
}
