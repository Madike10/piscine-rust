
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.to_lowercase();
    let mut s2 = s2.to_lowercase();
    
    // Convert the Strings to slices of characters and sort them
    s1.sort_unstable();
    s2.sort_unstable();
    
    s1 == s2
}