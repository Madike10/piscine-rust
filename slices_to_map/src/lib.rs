// Create a function that borrows two slices and returns a hashmap where the first slice represents the keys and the second represents the values.

//If the slices have different sizes, the function should return the hashmap with the size of the smallest list.


// Define the function with generic types T and U, and borrow the slices
use std::collections::HashMap;
use std::hash::Hash;

// Adjust the function signature to require T and U to implement Copy, Eq, and Hash
pub fn slices_to_map<'a, T: Copy + Eq + Hash, U: Copy>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {    let mut map = HashMap::new();

    // Determine the minimum length between the two slices
    let min_len = std::cmp::min(keys.len(), values.len());

    // Iterate over the slices up to the minimum length and insert into the map
    for i in 0..min_len {
        map.insert(&keys[i], &values[i]);
    }

    map
}
fn main() {
	let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
	let values = [1, 3, 23, 5, 2];
	println!("{:?}", slices_to_map(&keys, &values));
}

// And its output

// $ cargo run
// {"James": 2, "Liam": 3, "Emma": 23, "Noah": 5, "Olivia": 1}
// $
