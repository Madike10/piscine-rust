
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len(){
        return false;
    }
    
    let mut s1v :Vec<_> = s1.to_lowercase().chars().collect();
    let mut s2v :Vec<_> = s2.to_lowercase().chars().collect();
    
    // Convert the Strings to slices of characters and sort them
    s1v.sort();
    s2v.sort();
    
    if s1v == s2v {
        return true;
    }
    false
}