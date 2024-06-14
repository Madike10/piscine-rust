use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest = 0;
    for (_, value) in h {
        if value > biggest {
            biggest = value;
        }
    }
    return biggest;
}


fn main() {

    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!("The biggest of the elements in the HashMap is {}", bigger(hash));
}

// And its output

// $ cargo run
// The biggest of the elements in the HashMap is 334
// $
