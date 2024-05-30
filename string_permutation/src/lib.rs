pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<char> = s1.to_lowercase().chars().collect();
    let mut s2_chars: Vec<char> = s2.to_lowercase().chars().collect();

    s1_chars.sort();
    s2_chars.sort();

    s1_chars == s2_chars
}


fn main() {
	let word = "thought";
	let word1 = "thougth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
}

// And its output

// $ cargo run
// Is `thought` a permutation of `thougth`? = true
// $
