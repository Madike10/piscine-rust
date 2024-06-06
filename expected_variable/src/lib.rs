use std::cmp::max;


// Helper function to calculate the edit distance
fn edit_distance(a: &str, b: &str) -> usize {
    let mut costs = vec![0; b.len() + 1];

    for j in 0..=b.len() {
        costs[j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        costs[0] = i + 1;
        let mut nw = i;
        for (j, cb) in b.chars().enumerate() {
            let cj = costs[j];
            costs[j] = nw;
            nw = if ca == cb {
                cj
            } else {
                cj.min(nw).min(costs[j + 1]) + 1
            };
        }
    }

    costs[b.len()]
}

// Check if a string is in camel case
fn is_camel_case(s: &str) -> bool {
    s != s.to_lowercase() && s != s.to_uppercase() && !s.contains('_')
}

// Check if a string is in snake case
fn is_snake_case(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase() || c == '_') && s.contains('_')
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
