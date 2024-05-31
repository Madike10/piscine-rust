pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut s1 = s1.to_lowercase();
    let mut s2 = s2.to_lowercase();
    s1.sort();
    s2.sort();
    s1 == s2

}

 