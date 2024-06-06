use std::cmp::max;

// Helper function to calculate the edit distance
fn edit_distance(a: &str, b: &str) -> usize {
    let mut costs = vec![0; b.len() + 1];

    for j in 0..=b.len() {
        costs[j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        let mut last_value = i;
        costs[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let new_value = if ca == cb {
                last_value
            } else {
                min(last_value + 1, min(costs[j + 1] + 1, costs[j] + 1))
            };
            last_value = costs[j + 1];
            costs[j + 1] = new_value;
        }
    }

    costs[b.len()]
}

// Utility function to get the minimum of three values
fn min(a: usize, b: usize, c: usize) -> usize {
    a.min(b).min(c)
}

// Check if a string is in camel case
fn is_camel_case(s: &str) -> bool {
    let mut has_lower = false;
    let mut has_upper = false;
    let mut prev_is_upper = false;

    for c in s.chars() {
        if c.is_uppercase() {
            has_upper = true;
            if prev_is_upper {
                return false; // Two consecutive uppercase letters are not allowed
            }
            prev_is_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
            prev_is_upper = false;
        } else {
            return false; // Not a letter
        }
    }

    has_lower && has_upper
}

// Check if a string is in snake case
fn is_snake_case(s: &str) -> bool {
    s.split('_').all(|part| part.chars().all(|c| c.is_lowercase()))
}

// Main function to calculate expected variable similarity percentage
pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if !is_camel_case(compared) && !is_snake_case(compared) {
        return None;
    }

    let dist = edit_distance(&compared.to_lowercase(), &expected.to_lowercase());
    let similarity = 1.0 - dist as f64 / max(compared.len(), expected.len()) as f64;

    if similarity >= 0.5 {
        Some(format!("{:.0}%", similarity * 100.0))
    } else {
        None
    }
}