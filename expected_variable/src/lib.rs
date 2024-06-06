extern crate case;
use case::*;
use edit_distance::edit_distance;
// Assuming edit_distance is already implemented
// fn edit_distance(s1: &str, s2: &str) -> usize {
//     // Implementation goes here
// }

fn expected_variable(to_compare: &str, expected: &str) -> Option<String> {
    let to_compare = to_compare.to_lowercase();
    let expected = expected.to_lowercase();

    // Check if the string is in camel case or snake case
    if!is_camel_case(&to_compare) &&!is_snake_case(&to_compare) {
        return None;
    }

    // Calculate the edit distance
    let distance = edit_distance(&to_compare, &expected);

    // Calculate the similarity percentage
    let similarity_percentage = calculate_similarity_percentage(distance, expected.len());

    Some(format!("{}%", similarity_percentage))
}

fn is_camel_case(s: &str) -> bool {
    // Simple check for camel case
    s.contains('_') || s.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}

fn is_snake_case(s: &str) -> bool {
    // Simple check for snake case
    s.chars().all(|c| c.is_ascii_lowercase()) &&!s.contains('_')
}

fn calculate_similarity_percentage(distance: usize, expected_length: usize) -> f64 {
    let threshold = (expected_length as f64 * 0.5).round() as usize;
    if distance <= threshold {
        ((expected_length as f64 - distance as f64 / expected_length as f64) * 100.0).round() as f64
    } else {
        0.0
    }
}