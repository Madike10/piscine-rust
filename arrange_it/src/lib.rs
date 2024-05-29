pub fn arrange_phrase(phrase: &str) -> String {
    let mut words_with_positions = Vec::new();
    let mut start = 0;
    let mut has_digit = false;

    for (i, c) in phrase.char_indices() {
        if c.is_digit(10) {
            has_digit = true;
        } else if c == ' ' {
            if has_digit {
                words_with_positions.push((phrase[start..i].to_string(), start));
                has_digit = false;
            }
            start = i + 1;
        }
    }

    // Handle the last word if it has a digit
    if has_digit {
        words_with_positions.push((phrase[start..].to_string(), start));
    }

    // Sort words by the first digit found in each word
    words_with_positions.sort_by_key(|(word, _)| {
        word.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap()
    });

    // Collect and return the sorted words as a single string without digits
    let sorted_phrase: Vec<String> = words_with_positions.into_iter()
        .map(|(word, _)| word.chars().filter(|c| !c.is_digit(10)).collect())
        .collect();
    sorted_phrase.join(" ")
}
fn main() {
    println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}